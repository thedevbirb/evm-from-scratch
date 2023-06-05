use super::utils::types::{AccruedSubstate, GlobalState, Input, MachineState};

pub struct EVM {}

impl EVM {
    pub fn new() -> EVM {
        EVM {}
    }

    pub fn execute(
        mut global_state: GlobalState,
        mut machine_state: MachineState,
        mut accrued_substate: AccruedSubstate,
        mut input: Input,
    ) {
        dbg!("hello!");
    }
}
