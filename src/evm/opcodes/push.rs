use primitive_types::U256;

use crate::evm::utils::{
    constants::PUSH_HEX,
    errors::EVMError,
    types::{AccruedSubstate, GlobalState, Input, MachineState},
};

// 0x60 - 0x7f
pub fn push(
    _global_state: GlobalState,
    machine_state: MachineState,
    _accrued_substate: AccruedSubstate,
    input: Input,
) -> Result<(), EVMError> {
    let pc = machine_state.pc;
    let bytecode = input.bytecode;

    let offset: usize = (bytecode.get(pc).ok_or(EVMError::NoBytecode)? - PUSH_HEX).into();
    let data_position_hex: usize = (PUSH_HEX + 1).into();

    let data = bytecode
        .get(data_position_hex..data_position_hex + offset)
        .ok_or(EVMError::NoBytecode)?;

    let mut str_data = String::new();

    data.iter()
        .for_each(|byte| str_data.push_str(&format!("{:x}", byte)));

    let data = U256::from_str_radix(&str_data, 16).map_err(|_err| EVMError::FromStrRadix)?;

    Ok(())
}
