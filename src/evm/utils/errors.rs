use std::{
    backtrace::Backtrace,
    error,
    fmt::{self, write, Debug},
};

#[derive(Debug)]
pub enum EVMError {
    NoBytecodeError(NoBytecodeError),
    EmptyStack,
    FromStrRadix,
    NoOpcodeError(NoOpcodeError),
}

#[derive(Debug)]
pub struct NoBytecodeError {
    message: String,
    backtrace: Backtrace,
}

pub struct NoOpcodeError {
    message: String,
    expect: u8,
    backtrace: Backtrace,
}

impl NoBytecodeError {
    pub fn new() -> Self {
        Self {
            message: String::from("No bytecode found"),
            backtrace: Backtrace::force_capture(),
        }
    }
}

impl NoOpcodeError {
    pub fn new(opcode: u8) -> Self {
        Self {
            message: String::from("No opcode found"),
            expect: opcode,
            backtrace: Backtrace::force_capture(),
        }
    }
}

impl fmt::Debug for NoOpcodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "NoOpcodeError")
    }
}

impl fmt::Display for EVMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EVMError::NoBytecodeError(e) => {
                let msg = &e.message;
                let bt = &e.backtrace;
                write!(f, "{msg}: {bt}")
            }
            EVMError::NoOpcodeError(e) => {
                let msg = &e.message;
                let bt = &e.backtrace;
                let opcode = &e.expect;
                write!(f, "{msg} {opcode}: {bt}")
            }
            EVMError::EmptyStack => {
                write!(f, "empty stack")
            }
            EVMError::FromStrRadix => {
                write!(f, "could not convert hex string to U256")
            }
        }
    }
}

impl error::Error for EVMError {}
