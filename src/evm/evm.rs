use super::utils::{
    errors::EVMError,
    types::{AccruedSubstate, GlobalState, Input, MachineState},
};

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
    ) -> Result<(), EVMError> {
        dbg!("hello!");
        while machine_state.pc < input.bytecode.len() {
            let opcode = input
                .bytecode
                .get(machine_state.pc)
                .ok_or(EVMError::NoBytecode)?;

            // it is convenient to increment the pc after opcode execution
        }

        Ok(())
    }
}
