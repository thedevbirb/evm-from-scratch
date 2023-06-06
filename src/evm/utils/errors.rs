use std::{error, fmt};

#[derive(Debug)]
pub enum EVMError {
    NoBytecode,
    EmptyStack,
    FromStrRadix,
}

impl fmt::Display for EVMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let test = match f {
            Formatter<EVMError::NoBytecode> => {write!(f, "test")}
            _ => {
                write!(f, "test")
            }
        };
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EVMError {}
