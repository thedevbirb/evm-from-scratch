use std::{error::Error, fmt};

use super::types::ExecutionContext;

pub enum EVMError {
    NoBytecodeError(ExecutionContext),
    FromStrRadixError(String, ExecutionContext),
    NoOpcodeError(u8, ExecutionContext),
    EmptyStackError(ExecutionContext),
}

impl fmt::Display for EVMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EVMError::EmptyStackError(_) => {
                write!(f, "cannot pop from empty stack")
            }
            EVMError::NoOpcodeError(op, _) => {
                write!(f, "cannot find opcode {:x?}", op)
            }
            EVMError::NoBytecodeError(ctx) => {
                write!(f, "cannot find code at pc {}", ctx.machine_state.pc)
            }
            EVMError::FromStrRadixError(..) => {
                write!(f, "cannot parse string to hex")
            }
        }
    }
}

impl fmt::Debug for EVMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EVMError::NoBytecodeError(ctx) => {
                write!(f, "NoBytecodeError\n    ctx: {:#x?}", ctx)
            }
            EVMError::FromStrRadixError(str, ctx) => {
                write!(
                    f,
                    "FromStrRadixError\n    str: {},\n    ctx: {:#x?}",
                    str, ctx
                )
            }
            EVMError::NoOpcodeError(op, ctx) => {
                write!(f, "NoOpcodeError\n    op: {:x},\n    ctx: {:#x?}", op, ctx)
            }
            EVMError::EmptyStackError(ctx) => {
                write!(f, "EmptyStackError\n    ctx: {:#x?}", ctx)
            }
        }
    }
}

impl Error for EVMError {}
