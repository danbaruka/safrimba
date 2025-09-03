use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdResult, Deps, Binary, Addr, Uint128, Timestamp, BankMsg, Coin};
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, Action};
use crate::state::{TontineState, MemberProfile, DistributionMode, StartMode, StartConditionAuto};
use crate::storage::{save_state, load_state, save_member_contribution, load_member_contribution};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let admin: Addr = deps.api.addr_validate(&msg.admin)?;
    let members: Vec<Addr> = msg.members.iter().map(|s| deps.api.addr_validate(s)).collect::<Result<_, _>>()?;
    let member_profiles = match msg.member_profiles {
        Some(map) => {
            let mut out = std::collections::HashMap::new();
            for (k, v) in map.into_iter() {
                out.insert(deps.api.addr_validate(&k)?, v);
            }
            out
        }
        None => std::collections::HashMap::new(),
    };
    let distribution_calendar: Vec<Addr> = msg.distribution_calendar.iter().map(|s| deps.api.addr_validate(s)).collect::<Result<_, _>>()?;

    let state = TontineState {
        name: msg.name,
        symbol: msg.symbol,
        admin,
        members,
        member_profiles,
        contribution_amount: msg.contribution_amount,
        total_cycles: msg.total_cycles,
        cycle_duration: msg.cycle_duration,
        distribution_mode: msg.distribution_mode,
        start_mode: msg.start_mode,
        start_condition_auto: msg.start_condition_auto,
        deposit_deadline: msg.deposit_deadline,
        grace_seconds: msg.grace_seconds,
        late_penalty_percent: msg.late_penalty_percent,
        late_strike_limit: msg.late_strike_limit,
        distribution_calendar,
        allow_member_exit: msg.allow_member_exit,
        allow_member_add: msg.allow_member_add,
        early_withdrawal_penalty: msg.early_withdrawal_penalty,
        forbid_overpay: msg.forbid_overpay,
        forbid_underpay: msg.forbid_underpay,
        current_cycle: 0,
        completed: false,
        penalties_reserve: Uint128::zero(),
        member_strikes: std::collections::HashMap::new(),
        member_contributions: std::collections::HashMap::new(),
        calendar_locked: false,
        current_cycle_start: None,
        pending_exits: vec![],
        penalties_carry_over: Uint128::zero(),
        max_members: msg.max_members,
        caution_deposit: msg.caution_deposit,
        locked_caution: Uint128::zero(),
    };
    save_state(deps.storage, &state)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    let mut state = load_state(deps.storage)?;
    match msg.action {
        Action::AddMember { addr, pseudo } => {
            if state.current_cycle != 0 {
                return Err(cosmwasm_std::StdError::generic_err("Cannot add members after start"));
            }
            if state.members.len() as u32 >= state.max_members {
                return Err(cosmwasm_std::StdError::generic_err("Max members reached"));
            }
            // Require caution deposit in usaf
            let sent = info
                .funds
                .iter()
                .find(|c| c.denom == "usaf")
                .map(|c| c.amount)
                .unwrap_or_default();
            if sent < state.caution_deposit {
                return Err(cosmwasm_std::StdError::generic_err("Insufficient caution deposit"));
            }

            let addr = deps.api.addr_validate(&addr)?;
            if state.members.contains(&addr) {
                return Err(cosmwasm_std::StdError::generic_err("Member already exists"));
            }
            state.members.push(addr.clone());
            state.member_profiles.insert(addr.clone(), MemberProfile { pseudo });
            state.locked_caution += state.caution_deposit;
            save_state(deps.storage, &state)?;
            Ok(Response::new().add_attribute("action", "add_member"))
        }
        Action::RemoveMember { addr } => {
            if info.sender != state.admin {
                return Err(cosmwasm_std::StdError::generic_err("Only admin can remove members"));
            }
            let addr = deps.api.addr_validate(&addr)?;
            state.members.retain(|m| m != &addr);
            state.member_profiles.remove(&addr);
            save_state(deps.storage, &state)?;
            Ok(Response::new().add_attribute("action", "remove_member"))
        }
        Action::UpdateProfile { addr, pseudo } => {
            if info.sender != state.admin {
                return Err(cosmwasm_std::StdError::generic_err("Only admin can update profiles"));
            }
            if state.current_cycle != 0 {
                return Err(cosmwasm_std::StdError::generic_err("Cannot update profiles after start"));
            }
            let addr = deps.api.addr_validate(&addr)?;
            if !state.members.contains(&addr) {
                return Err(cosmwasm_std::StdError::generic_err("Member not found"));
            }
            let entry = state.member_profiles.entry(addr.clone()).or_insert(MemberProfile { pseudo: None });
            entry.pseudo = pseudo;
            save_state(deps.storage, &state)?;
            Ok(Response::new().add_attribute("action", "update_profile"))
        }
        Action::Deposit { cycle } => {
            if !state.members.contains(&info.sender) {
                return Err(cosmwasm_std::StdError::generic_err("Not a member"));
            }
            if cycle != state.current_cycle {
                return Err(cosmwasm_std::StdError::generic_err("Invalid cycle"));
            }
            if state.current_cycle == 0 {
                return Err(cosmwasm_std::StdError::generic_err("Tontine not started"));
            }
            let now = env.block.time;
            let start = state.current_cycle_start.ok_or_else(|| cosmwasm_std::StdError::generic_err("Cycle start not set"))?;
            let deadline = Timestamp::from_nanos(start.nanos() + (state.deposit_deadline as u64) * 1_000_000_000);
            let grace_end = Timestamp::from_nanos(deadline.nanos() + (state.grace_seconds as u64) * 1_000_000_000);

            let sent = info.funds.iter().find(|c| c.denom == "usaf").map(|c| c.amount).unwrap_or_default();
            if state.forbid_overpay && sent > state.contribution_amount {
                return Err(cosmwasm_std::StdError::generic_err("Overpayment not allowed"));
            }
            if state.forbid_underpay && sent < state.contribution_amount {
                return Err(cosmwasm_std::StdError::generic_err("Underpayment not allowed"));
            }

            if now > grace_end {
                return Err(cosmwasm_std::StdError::generic_err("Deposit window closed"));
            }

            save_member_contribution(deps.storage, &info.sender, cycle, sent.u128())?;

            if now > deadline && now <= grace_end {
                let strikes = state.member_strikes.entry(info.sender.clone()).or_insert(0);
                *strikes += 1;
                let penalty = state.late_penalty_percent as u128 * state.contribution_amount.u128() / 100u128;
                state.penalties_reserve += Uint128::from(penalty);
                if *strikes > state.late_strike_limit {
                    state.members.retain(|m| m != &info.sender);
                }
            }

            save_state(deps.storage, &state)?;
            Ok(Response::new().add_attribute("action", "deposit"))
        }
        Action::InitiateStart {} => {
            if state.start_mode != StartMode::Manual {
                return Err(cosmwasm_std::StdError::generic_err("Manual start not allowed"));
            }
            if info.sender != state.admin {
                return Err(cosmwasm_std::StdError::generic_err("Only admin can start"));
            }
            if state.current_cycle != 0 {
                return Err(cosmwasm_std::StdError::generic_err("Already started"));
            }
            state.current_cycle = 1;
            state.current_cycle_start = Some(env.block.time);
            state.calendar_locked = true;
            save_state(deps.storage, &state)?;
            Ok(Response::new().add_attribute("action", "initiate_start"))
        }
        Action::AutoStart {} => {
            if state.start_mode != StartMode::Auto {
                return Err(cosmwasm_std::StdError::generic_err("Auto start not allowed"));
            }
            if state.current_cycle != 0 {
                return Err(cosmwasm_std::StdError::generic_err("Already started"));
            }
            let can_start = match &state.start_condition_auto {
                Some(StartConditionAuto::MembersReached(n)) => state.members.len() as u32 >= *n,
                Some(StartConditionAuto::StartDate(ts)) => env.block.time >= *ts,
                _ => false,
            };
            if !can_start {
                return Err(cosmwasm_std::StdError::generic_err("Auto start conditions not met"));
            }
            state.current_cycle = 1;
            state.current_cycle_start = Some(env.block.time);
            state.calendar_locked = true;
            save_state(deps.storage, &state)?;
            Ok(Response::new().add_attribute("action", "auto_start"))
        }
        Action::ComputeDistributionCalendar {} => {
            if info.sender != state.admin {
                return Err(cosmwasm_std::StdError::generic_err("Only admin can compute calendar"));
            }
            if state.calendar_locked {
                return Err(cosmwasm_std::StdError::generic_err("Calendar is locked"));
            }
            match state.distribution_mode {
                DistributionMode::Fifo => {
                    state.distribution_calendar = state.members.clone();
                }
                DistributionMode::Random => {
                    let mut members = state.members.clone();
                    if !members.is_empty() {
                        let seed = env.block.height as usize % members.len();
                        members.rotate_left(seed);
                    }
                    state.distribution_calendar = members;
                }
                DistributionMode::Custom => {}
            }
            save_state(deps.storage, &state)?;
            Ok(Response::new().add_attribute("action", "compute_distribution_calendar"))
        }
        Action::RecordLatePayment { addr } => {
            let member = deps.api.addr_validate(&addr)?;
            let strikes = state.member_strikes.entry(member.clone()).or_insert(0);
            *strikes += 1;
            if *strikes > state.late_strike_limit {
                state.members.retain(|m| m != &member);
            }
            let penalty = state.late_penalty_percent as u128 * state.contribution_amount.u128() / 100u128;
            state.penalties_reserve += Uint128::from(penalty);
            save_state(deps.storage, &state)?;
            Ok(Response::new().add_attribute("action", "record_late_payment"))
        }
        Action::ApplyPenalty { addr } => {
            let _member = deps.api.addr_validate(&addr)?;
            let penalty = state.late_penalty_percent as u128 * state.contribution_amount.u128() / 100u128;
            state.penalties_reserve += Uint128::from(penalty);
            save_state(deps.storage, &state)?;
            Ok(Response::new().add_attribute("action", "apply_penalty"))
        }
        Action::TriggerPayout {} => {
            let cycle = state.current_cycle;
            if info.sender != state.admin {
                return Err(cosmwasm_std::StdError::generic_err("Only admin can trigger payout"));
            }
            if cycle != state.current_cycle {
                return Err(cosmwasm_std::StdError::generic_err("Invalid cycle for payout"));
            }
            if state.distribution_calendar.len() < state.members.len() {
                return Err(cosmwasm_std::StdError::generic_err("Distribution calendar incomplete"));
            }
            for member in &state.members {
                let paid = load_member_contribution(deps.storage, member, cycle)?;
                if paid.is_none() {
                    return Err(cosmwasm_std::StdError::generic_err("Missing contribution"));
                }
            }
            let mut total = Uint128::zero();
            for member in &state.members {
                let paid = load_member_contribution(deps.storage, member, cycle)?.unwrap_or(0u128);
                total += Uint128::from(paid);
            }
            total += state.penalties_reserve + state.penalties_carry_over;
            let idx = (cycle - 1) as usize;
            let recipient = state.distribution_calendar.get(idx).ok_or_else(|| cosmwasm_std::StdError::generic_err("Recipient not found"))?;
            let send = BankMsg::Send { to_address: recipient.to_string(), amount: vec![Coin { denom: "usaf".to_string(), amount: total }] };
            state.current_cycle += 1;
            state.current_cycle_start = Some(env.block.time);
            state.penalties_reserve = Uint128::zero();
            state.penalties_carry_over = Uint128::zero();
            if state.current_cycle > state.total_cycles { state.completed = true; }
            save_state(deps.storage, &state)?;
            Ok(Response::new().add_message(send).add_attribute("action", "trigger_payout"))
        }
        Action::DistributePenalties {} => {
            if info.sender != state.admin { return Err(cosmwasm_std::StdError::generic_err("Only admin can distribute penalties")); }
            if !state.members.is_empty() {
                state.penalties_carry_over += state.penalties_reserve;
                state.penalties_reserve = Uint128::zero();
            }
            save_state(deps.storage, &state)?;
            Ok(Response::new().add_attribute("action", "distribute_penalties"))
        }
        Action::RequestExit {} => {
            if !state.allow_member_exit { return Err(cosmwasm_std::StdError::generic_err("Member exit not allowed")); }
            if !state.members.contains(&info.sender) { return Err(cosmwasm_std::StdError::generic_err("Not a member")); }
            if state.pending_exits.contains(&info.sender) { return Err(cosmwasm_std::StdError::generic_err("Exit already requested")); }
            state.pending_exits.push(info.sender.clone());
            save_state(deps.storage, &state)?;
            Ok(Response::new().add_attribute("action", "request_exit"))
        }
        Action::ProcessExit {} => {
            if info.sender != state.admin { return Err(cosmwasm_std::StdError::generic_err("Only admin can process exit")); }
            let remaining: Vec<Addr> = vec![];
            for addr in state.pending_exits.iter() {
                if state.members.contains(addr) {
                    let penalty = state.early_withdrawal_penalty as u128 * state.contribution_amount.u128() / 100u128;
                    state.penalties_reserve += Uint128::from(penalty);
                    state.members.retain(|m| m != addr);
                }
            }
            state.pending_exits = remaining;
            save_state(deps.storage, &state)?;
            Ok(Response::new().add_attribute("action", "process_exit"))
        }
        Action::ReplaceMember { old_addr, new_addr } => {
            if info.sender != state.admin { return Err(cosmwasm_std::StdError::generic_err("Only admin can replace member")); }
            let old_addr = deps.api.addr_validate(&old_addr)?;
            let new_addr = deps.api.addr_validate(&new_addr)?;
            state.members.retain(|m| m != &old_addr);
            state.members.push(new_addr.clone());
            save_state(deps.storage, &state)?;
            Ok(Response::new().add_attribute("action", "replace_member"))
        }
        Action::ViewCalendar {} => {
            Ok(Response::new().add_attribute("calendar", format!("{:?}", state.distribution_calendar)))
        }
    }
}

#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    let state = load_state(deps.storage)?;
    match msg {
        QueryMsg::GetGroupInfo {} => {
            let members: Vec<String> = state.members.iter().map(|a| a.to_string()).collect();
            let distribution_calendar: Vec<String> = state.distribution_calendar.iter().map(|a| a.to_string()).collect();
            let strikes: Vec<(String, u8)> = state.member_strikes.iter().map(|(addr, s)| (addr.to_string(), *s)).collect();
            let profiles: Vec<(String, MemberProfile)> = state.member_profiles.iter().map(|(addr, p)| (addr.to_string(), p.clone())).collect();
            #[derive(serde::Serialize)]
            struct GroupInfoDTO {
                name: String,
                symbol: String,
                admin: String,
                members: Vec<String>,
                member_profiles: Vec<(String, MemberProfile)>,
                contribution_amount: String,
                total_cycles: u32,
                cycle_duration: u64,
                distribution_mode: DistributionMode,
                start_mode: StartMode,
                start_condition_auto: Option<StartConditionAuto>,
                deposit_deadline: u64,
                grace_seconds: u32,
                late_penalty_percent: u8,
                late_strike_limit: u8,
                distribution_calendar: Vec<String>,
                allow_member_exit: bool,
                allow_member_add: bool,
                early_withdrawal_penalty: u8,
                forbid_overpay: bool,
                forbid_underpay: bool,
                current_cycle: u32,
                completed: bool,
                penalties_reserve: String,
                strikes: Vec<(String, u8)>,
                calendar_locked: bool,
                current_cycle_start: Option<Timestamp>,
                max_members: u32,
                caution_deposit: Uint128,
                locked_caution: Uint128,
            }
            let dto = GroupInfoDTO {
                name: state.name.clone(),
                symbol: state.symbol.clone(),
                admin: state.admin.to_string(),
                members,
                member_profiles: profiles,
                contribution_amount: state.contribution_amount.to_string(),
                total_cycles: state.total_cycles,
                cycle_duration: state.cycle_duration,
                distribution_mode: state.distribution_mode.clone(),
                start_mode: state.start_mode.clone(),
                start_condition_auto: state.start_condition_auto.clone(),
                deposit_deadline: state.deposit_deadline,
                grace_seconds: state.grace_seconds,
                late_penalty_percent: state.late_penalty_percent,
                late_strike_limit: state.late_strike_limit,
                distribution_calendar,
                allow_member_exit: state.allow_member_exit,
                allow_member_add: state.allow_member_add,
                early_withdrawal_penalty: state.early_withdrawal_penalty,
                forbid_overpay: state.forbid_overpay,
                forbid_underpay: state.forbid_underpay,
                current_cycle: state.current_cycle,
                completed: state.completed,
                penalties_reserve: state.penalties_reserve.to_string(),
                strikes,
                calendar_locked: state.calendar_locked,
                current_cycle_start: state.current_cycle_start,
                max_members: state.max_members,
                caution_deposit: state.caution_deposit,
                locked_caution: state.locked_caution,
            };
            let resp = serde_json_wasm::to_vec(&dto).map_err(|e| cosmwasm_std::StdError::generic_err(e.to_string()))?;
            Ok(Binary(resp))
        }
        QueryMsg::GetMemberInfo { addr } => {
            let addr = deps.api.addr_validate(&addr)?;
            let profile_owned: Option<MemberProfile> = state.member_profiles.get(&addr).cloned();
            let strikes = state.member_strikes.get(&addr).cloned().unwrap_or(0);
            let mut contribs: Vec<(u32, Uint128)> = vec![];
            for cycle in 1..=state.current_cycle.max(1) {
                if let Some(amt) = state.member_contributions.get(&(addr.clone(), cycle)) {
                    contribs.push((cycle, *amt));
                }
            }
            #[derive(serde::Serialize)]
            struct MemberInfoDTO { addr: String, profile: Option<MemberProfile>, strikes: u8, contributions: Vec<(u32, Uint128)> }
            let dto = MemberInfoDTO { addr: addr.to_string(), profile: profile_owned, strikes, contributions: contribs };
            let resp = serde_json_wasm::to_vec(&dto).map_err(|e| cosmwasm_std::StdError::generic_err(e.to_string()))?;
            Ok(Binary(resp))
        }
        QueryMsg::GetCycleInfo { cycle } => {
            let recipient = state.distribution_calendar.get((cycle.saturating_sub(1)) as usize).cloned();
            let mut total = Uint128::zero();
            for member in &state.members {
                if let Some(amt) = state.member_contributions.get(&(member.clone(), cycle)) { total += *amt; }
            }
            let penalties = state.penalties_reserve + state.penalties_carry_over;
            let status = if cycle < state.current_cycle { "closed" } else if cycle == state.current_cycle { "open" } else { "future" };
            #[derive(serde::Serialize)]
            struct CycleInfoDTO { cycle: u32, status: String, recipient: Option<String>, total_contributions: Uint128, penalties: Uint128 }
            let dto = CycleInfoDTO { cycle, status: status.to_string(), recipient: recipient.map(|a| a.to_string()), total_contributions: total, penalties };
            let resp = serde_json_wasm::to_vec(&dto).map_err(|e| cosmwasm_std::StdError::generic_err(e.to_string()))?;
            Ok(Binary(resp))
        }
        QueryMsg::GetDistributionCalendar {} => {
            let cal: Vec<String> = state.distribution_calendar.iter().map(|a| a.to_string()).collect();
            let resp = serde_json_wasm::to_vec(&cal).map_err(|e| cosmwasm_std::StdError::generic_err(e.to_string()))?;
            Ok(Binary(resp))
        }
    }
}
