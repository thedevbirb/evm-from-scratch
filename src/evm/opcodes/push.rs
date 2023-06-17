use primitive_types::U256;

use crate::evm::utils::{
    constants::{PUSH_0_HEX, PUSH_1_HEX},
    errors::EVMError,
    types::{ExecutionContext, OpcodeResult},
};

// 0x59 - 0x7f
pub fn push(ctx: &mut ExecutionContext) -> OpcodeResult {
    let pc = ctx.machine_state.pc;
    let bytecode = &ctx.input.bytecode;
    let opcode = bytecode
        .get(pc)
        .ok_or(EVMError::NoBytecodeError(ctx.clone()))?
        .clone();

    if opcode == PUSH_0_HEX {
        ctx.machine_state.stack.push(U256::zero());
        return Ok(());
    }

    let offset: usize = (opcode - PUSH_1_HEX + 1).into();
    let data_position_hex: usize = pc + 1;

    let data = bytecode
        .get(data_position_hex..data_position_hex + offset)
        .ok_or(EVMError::NoBytecodeError(ctx.clone()))?;

    ctx.machine_state.pc += data.len();

    let mut str_data = String::new();

    data.iter()
        .for_each(|byte| str_data.push_str(&format!("{:x}", byte)));

    let data = U256::from_str_radix(&str_data, 16)
        .map_err(|_err| EVMError::FromStrRadixError(str_data, ctx.clone()))?;

    ctx.machine_state.stack.push(data);

    Ok(())
}
