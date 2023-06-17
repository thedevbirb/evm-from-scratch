use crate::evm::utils::types::{AccruedSubstate, GlobalState, Input, MachineState, OpcodeResult};

// 0x00
pub fn stop(
    _global_state: &mut GlobalState,
    _machine_state: &mut MachineState,
    _accrued_substate: &mut AccruedSubstate,
    _input: &mut Input,
) -> OpcodeResult {
    Ok(())
}
