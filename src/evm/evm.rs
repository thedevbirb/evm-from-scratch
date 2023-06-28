use crate::evm::utils::constants::REVERT;

use super::utils::constants::{INVALID, NO_STATIC_OPCODES};
use super::utils::types::EVMReturnData;
use super::utils::{errors::EVMError, helpers::get_opcodes, types::ExecutionContext};

pub struct EVM {}

impl EVM {
    pub fn execute(ctx: &mut ExecutionContext) -> Result<EVMReturnData, EVMError> {
        let opcodes = get_opcodes();
        let mut output = None;
        let mut success = true;

        while ctx.machine_state.pc < ctx.input.bytecode.len() {
            let opcode = ctx
                .input
                .bytecode
                .get(ctx.machine_state.pc)
                .ok_or(EVMError::NoBytecodeError(ctx.clone()))?;

            match *opcode {
                REVERT => {
                    success = false;
                }
                INVALID => {
                    success = false;
                    break;
                }
                _ => {}
            }

            if NO_STATIC_OPCODES.contains(opcode) && !ctx.input.write {
                success = false;
                break;
            }

            let runner = opcodes
                .get(opcode)
                .ok_or(EVMError::NoOpcodeError(*opcode, ctx.clone()))?;

            output = match runner(ctx) {
                Ok(option) => option,
                Err(e) => match e {
                    EVMError::InvalidJumpdestError(_, _) => {
                        success = false;
                        break;
                    }
                    _ => return Err(e),
                },
            };

            ctx.machine_state.pc += 1;

            if let Some(_data) = &output {
                break;
            }
        }

        ctx.accrued_substate
            .self_destruct_set
            .iter()
            .for_each(|account| {
                ctx.global_state.remove(account);
            });

        Ok(EVMReturnData { success, output })
    }
}
