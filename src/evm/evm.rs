use crate::evm::utils::types::Output;

use super::utils::{
    errors::{EVMError, NoBytecodeError, NoOpcodeError},
    helpers::get_opcodes,
    types::{AccruedSubstate, EVMReturnData, GlobalState, Input, MachineState},
};

pub struct EVM {}

impl EVM {
    pub fn execute(
        mut global_state: GlobalState,
        mut machine_state: MachineState,
        mut accrued_substate: AccruedSubstate,
        mut input: Input,
    ) -> Result<EVMReturnData, EVMError> {
        let opcodes = get_opcodes();

        while machine_state.pc < input.bytecode.len() {
            let opcode = input
                .bytecode
                .get(machine_state.pc)
                .ok_or(EVMError::NoBytecodeError(NoBytecodeError::new()))?;

            let runner = opcodes
                .get(opcode)
                .ok_or(EVMError::NoOpcodeError(NoOpcodeError::new(*opcode)))?;

            runner(
                &mut global_state,
                &mut machine_state,
                &mut accrued_substate,
                &mut input,
            )?;

            machine_state.pc += 1;
        }

        Ok(EVMReturnData {
            global_state,
            machine_state,
            accrued_substate,
            output: Output { success: true },
        })
    }
}
