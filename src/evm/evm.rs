use crate::evm::utils::types::Output;

use super::utils::constants::STOP;
use super::utils::types::EVMReturnData;
use super::utils::{errors::EVMError, helpers::get_opcodes, types::ExecutionContext};

pub struct EVM {}

impl EVM {
    pub fn execute(mut ctx: ExecutionContext) -> Result<EVMReturnData, EVMError> {
        let opcodes = get_opcodes();

        while ctx.machine_state.pc < ctx.input.bytecode.len() {
            let opcode = ctx
                .input
                .bytecode
                .get(ctx.machine_state.pc)
                .ok_or(EVMError::NoBytecodeError(ctx.clone()))?;

            if opcode == &STOP {
                break;
            }

            let runner = opcodes
                .get(opcode)
                .ok_or(EVMError::NoOpcodeError(*opcode, ctx.clone()))?;

            runner(&mut ctx)?;

            ctx.machine_state.pc += 1;
        }

        Ok(EVMReturnData {
            ctx,
            output: Output { success: true },
        })
    }
}
