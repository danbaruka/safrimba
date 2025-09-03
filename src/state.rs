use cosmwasm_std::{Addr, Uint128, Timestamp};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TontineState {
    pub name: String,
    pub symbol: String,
    pub admin: Addr,
    pub members: Vec<Addr>,
    pub member_profiles: HashMap<Addr, MemberProfile>,
    pub contribution_amount: Uint128,
    pub total_cycles: u32,
    pub cycle_duration: u64,
    pub distribution_mode: DistributionMode,
    pub start_mode: StartMode,
    pub start_condition_auto: Option<StartConditionAuto>,
    pub deposit_deadline: u64,
    pub grace_seconds: u32,
    pub late_penalty_percent: u8,
    pub late_strike_limit: u8,
    pub distribution_calendar: Vec<Addr>,
    pub allow_member_exit: bool,
    pub allow_member_add: bool,
    pub early_withdrawal_penalty: u8,
    pub forbid_overpay: bool,
    pub forbid_underpay: bool,
    pub current_cycle: u32,
    pub completed: bool,
    pub penalties_reserve: Uint128,
    pub member_strikes: HashMap<Addr, u8>,
    pub member_contributions: HashMap<(Addr, u32), Uint128>,
    pub calendar_locked: bool,
    pub current_cycle_start: Option<Timestamp>,
    pub pending_exits: Vec<Addr>,
    pub penalties_carry_over: Uint128,
    pub max_members: u32,
    pub caution_deposit: Uint128,
    pub locked_caution: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MemberProfile {
    pub pseudo: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum DistributionMode {
    Fifo,
    Random,
    Custom,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum StartMode {
    Manual,
    Auto,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum StartConditionAuto {
    MembersReached(u32),
    StartDate(Timestamp),
}
