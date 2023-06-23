use primitive_types::U256;

use crate::evm::utils::{
    constants::{PUSH_0, PUSH_1},
    errors::EVMError,
    helpers::hex_string_from_bytes,
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

    if opcode == PUSH_0 {
        ctx.machine_state.stack.push(U256::zero());
        return Ok(None);
    }

    let offset: usize = (opcode - PUSH_1 + 1).into();
    let data_position_hex: usize = pc + 1;

    let data = bytecode
        .get(data_position_hex..data_position_hex + offset)
        .ok_or(EVMError::NoBytecodeError(ctx.clone()))?;

    ctx.machine_state.pc += data.len();

    let str_data = hex_string_from_bytes(data);

    let data = U256::from_str_radix(&str_data, 16)
        .map_err(|_err| EVMError::FromStrRadixError(str_data, ctx.clone()))?;

    ctx.machine_state.stack.push(data);

    Ok(None)
}
