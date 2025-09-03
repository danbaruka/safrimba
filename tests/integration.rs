use cosmwasm_std::Addr;
use cw_multi_test::{App, ContractWrapper, Executor};
use safrimba_smartcontract::msg::{InstantiateMsg, ExecuteMsg, Action};


#[test]
fn test_instantiate_add_member_and_deposit() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        safrimba_smartcontract::contract::execute,
        safrimba_smartcontract::contract::instantiate,
        safrimba_smartcontract::contract::query,
    );
    let code_id = app.store_code(Box::new(code));
    let admin = Addr::unchecked("addr_safro1admin");
    let member = Addr::unchecked("addr_safro1member");
    let msg = InstantiateMsg {
        name: "Test Group".to_string(),
        symbol: "TST".to_string(),
        admin: admin.clone(),
        members: vec![admin.clone()],
        member_profiles: None,
        contribution_amount: 100u128.into(),
        total_cycles: 5,
        cycle_duration: 30 * 24 * 60 * 60,
        distribution_mode: safrimba_smartcontract::msg::DistributionMode::Fifo,
        start_mode: safrimba_smartcontract::msg::StartMode::Manual,
        start_condition_auto: None,
        deposit_deadline: 24 * 60 * 60,
        grace_seconds: 3600,
        late_penalty_percent: 10,
        late_strike_limit: 3,
        distribution_calendar: vec![admin.clone()],
        allow_member_exit: true,
        allow_member_add: true,
        early_withdrawal_penalty: 5,
        forbid_overpay: true,
        forbid_underpay: true,
    };
    let contract_addr = app.instantiate_contract(code_id, admin.clone(), &msg, &[], "Safrimba", None).unwrap();
    let add_member_msg = ExecuteMsg {
        action: Action::AddMember { addr: member.clone(), pseudo: Some("Member1".to_string()), profile: Some("desc".to_string()) },
    };
    let _ = app.execute_contract(admin.clone(), contract_addr.clone(), &add_member_msg, &[]).unwrap();

    // Initiate start
    let start_msg = ExecuteMsg {
        action: Action::InitiateStart {},
    };
    let _ = app.execute_contract(admin.clone(), contract_addr.clone(), &start_msg, &[]).unwrap();

    // Deposit for cycle 1
    let deposit_msg = ExecuteMsg {
        action: Action::Deposit { cycle: 1 },
    };
    let _ = app.execute_contract(member.clone(), contract_addr.clone(), &deposit_msg, &[]).unwrap();
}
