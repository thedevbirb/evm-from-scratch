use crate::evm::utils::types::{GlobalState, MachineState, AccruedSubstate, Input, OpcodeResult};

// 0x50
pub fn pop(
    _global_state: &mut GlobalState,
    machine_state: &mut MachineState,
    _accrued_substate: &mut AccruedSubstate,
    input: &mut Input,
) -> OpcodeResult {
    Ok(())
}
