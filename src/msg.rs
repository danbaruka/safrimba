
use cosmwasm_std::{Uint128};
use std::collections::HashMap;
use crate::state::{MemberProfile, DistributionMode, StartMode, StartConditionAuto};
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub admin: String,
    pub members: Vec<String>,
    pub member_profiles: Option<HashMap<String, MemberProfile>>, // Optional, can be empty
    pub contribution_amount: Uint128,
    pub total_cycles: u32,
    pub cycle_duration: u64, // seconds
    pub distribution_mode: DistributionMode,
    pub start_mode: StartMode,
    pub start_condition_auto: Option<StartConditionAuto>,
    pub deposit_deadline: u64, // seconds
    pub grace_seconds: u32,
    pub late_penalty_percent: u8,
    pub late_strike_limit: u8,
    pub distribution_calendar: Vec<String>,
    pub allow_member_exit: bool,
    pub allow_member_add: bool,
    pub early_withdrawal_penalty: u8,
    pub forbid_overpay: bool,
    pub forbid_underpay: bool,
    pub max_members: u32,
    pub caution_deposit: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExecuteMsg {
    pub action: Action,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Action {
    AddMember { addr: String, pseudo: Option<String> },
    RemoveMember { addr: String },
    UpdateProfile { addr: String, pseudo: Option<String> },
    InitiateStart {},
    AutoStart {},
    ComputeDistributionCalendar {},
    Deposit { cycle: u32 },
    RecordLatePayment { addr: String },
    ApplyPenalty { addr: String },
    TriggerPayout {},
    DistributePenalties {},
    ViewCalendar {},
    RequestExit {},
    ProcessExit {},
    ReplaceMember { old_addr: String, new_addr: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryMsg {
    pub query: Query,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Query {
    GetGroupInfo {},
    GetMemberInfo { addr: String },
    GetCycleInfo { cycle: u32 },
    GetDistributionCalendar {},
}
