use cosmwasm_std::{Storage, StdResult, Addr};
use crate::state::TontineState;

// Storage keys
pub const STATE_KEY: &[u8] = b"state";
pub const MEMBER_STRIKES_PREFIX: &[u8] = b"member_strikes";
pub const MEMBER_CONTRIBUTIONS_PREFIX: &[u8] = b"member_contributions";

// Helper functions for storage operations
pub fn save_state(store: &mut dyn Storage, state: &TontineState) -> StdResult<()> {
    let data = serde_json_wasm::to_vec(state).map_err(|e| cosmwasm_std::StdError::serialize_err("TontineState", e))?;
    store.set(STATE_KEY, &data);
    Ok(())
}

pub fn load_state(store: &dyn Storage) -> StdResult<TontineState> {
    let data = store.get(STATE_KEY).ok_or_else(|| cosmwasm_std::StdError::not_found("state"))?;
    serde_json_wasm::from_slice(&data).map_err(|e| cosmwasm_std::StdError::parse_err("TontineState", e))
}

pub fn save_member_strikes(store: &mut dyn Storage, addr: &Addr, strikes: u8) -> StdResult<()> {
    let key = [MEMBER_STRIKES_PREFIX, addr.as_bytes()].concat();
    let data = serde_json_wasm::to_vec(&strikes).map_err(|e| cosmwasm_std::StdError::serialize_err("u8", e))?;
    store.set(&key, &data);
    Ok(())
}

pub fn load_member_strikes(store: &dyn Storage, addr: &Addr) -> StdResult<Option<u8>> {
    let key = [MEMBER_STRIKES_PREFIX, addr.as_bytes()].concat();
    if let Some(data) = store.get(&key) {
        Ok(Some(serde_json_wasm::from_slice(&data).map_err(|e| cosmwasm_std::StdError::parse_err("u8", e))?))
    } else {
        Ok(None)
    }
}

pub fn save_member_contribution(store: &mut dyn Storage, addr: &Addr, cycle: u32, amount: u128) -> StdResult<()> {
    let key = [MEMBER_CONTRIBUTIONS_PREFIX, addr.as_bytes(), &cycle.to_be_bytes()].concat();
    let data = serde_json_wasm::to_vec(&amount).map_err(|e| cosmwasm_std::StdError::serialize_err("u128", e))?;
    store.set(&key, &data);
    Ok(())
}

pub fn load_member_contribution(store: &dyn Storage, addr: &Addr, cycle: u32) -> StdResult<Option<u128>> {
    let key = [MEMBER_CONTRIBUTIONS_PREFIX, addr.as_bytes(), &cycle.to_be_bytes()].concat();
    if let Some(data) = store.get(&key) {
        Ok(Some(serde_json_wasm::from_slice(&data).map_err(|e| cosmwasm_std::StdError::parse_err("u128", e))?))
    } else {
        Ok(None)
    }
}
