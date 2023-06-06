use primitive_types::U256;

use super::types::{Address, GlobalState};

/// Models the EMPTY function in the yellow paper
pub fn is_account_empty(state: GlobalState, address: Address) -> bool {
    if let Some(account_state) = state.get(&address) {
        account_state.nonce == 0 && account_state.balance.is_zero()
    } else {
        false
    }
}

/// Models the DEAD function in the yellow paper
pub fn is_account_dead(state: GlobalState, address: Address) -> bool {
    state.get(&address).is_none() || is_account_empty(state, address)
}
