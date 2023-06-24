use crate::evm::utils::constants::REVERT;

use super::utils::constants::{NO_STATIC_OPCODES, STOP};
use super::utils::types::EVMReturnData;
use super::utils::{errors::EVMError, helpers::get_opcodes, types::ExecutionContext};

pub struct EVM {}

impl EVM {
    pub fn execute(ctx: &mut ExecutionContext) -> Result<EVMReturnData, EVMError> {
        let opcodes = get_opcodes();
        let mut output = None;
        let mut reverted = false;

        while ctx.machine_state.pc < ctx.input.bytecode.len() {
            let opcode = ctx
                .input
                .bytecode
                .get(ctx.machine_state.pc)
                .ok_or(EVMError::NoBytecodeError(ctx.clone()))?;

            if opcode == &STOP {
                break;
            } else if opcode == &REVERT {
                reverted = true;
            } else if NO_STATIC_OPCODES.contains(opcode) && !ctx.input.write {
                reverted = true;
                break;
            }

            let runner = opcodes
                .get(opcode)
                .ok_or(EVMError::NoOpcodeError(*opcode, ctx.clone()))?;

            output = runner(ctx)?;

            ctx.machine_state.pc += 1;

            if let Some(_data) = &output {
                break;
            }
        }

        Ok(EVMReturnData {
            success: !reverted,
            output,
        })
    }
}
