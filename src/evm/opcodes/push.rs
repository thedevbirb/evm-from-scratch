use primitive_types::U256;

use crate::evm::utils::{
    constants::{PUSH_0_HEX, PUSH_1_HEX},
    errors::{EVMError, NoBytecodeError},
    types::{AccruedSubstate, GlobalState, Input, MachineState, OpcodeResult},
};

// 0x59 - 0x7f
pub fn push(
    _global_state: &mut GlobalState,
    machine_state: &mut MachineState,
    _accrued_substate: &mut AccruedSubstate,
    input: &mut Input,
) -> OpcodeResult {
    let pc = machine_state.pc;
    let bytecode = &input.bytecode;
    let opcode = bytecode
        .get(pc)
        .ok_or(EVMError::NoBytecodeError(NoBytecodeError::new()))?
        .clone();

    if opcode == PUSH_0_HEX {
        machine_state.stack.push(U256::zero());
        return Ok(());
    }

    let offset: usize = (opcode - PUSH_1_HEX + 1).into();
    let data_position_hex: usize = pc + 1;

    let data = bytecode
        .get(data_position_hex..data_position_hex + offset)
        .ok_or(EVMError::NoBytecodeError(NoBytecodeError::new()))?;

    machine_state.pc += data.len();

    let mut str_data = String::new();

    data.iter()
        .for_each(|byte| str_data.push_str(&format!("{:x}", byte)));

    let data = U256::from_str_radix(&str_data, 16).map_err(|_err| EVMError::FromStrRadix)?;

    machine_state.stack.push(data);

    Ok(())
}
