use primitive_types::U256;

use crate::evm::utils::{
    constants::BYTES_IN_U256_FROM_ZERO,
    errors::EVMError,
    helpers::{hex_string_from_byte, pop_n},
    types::{ExecutionContext, OpcodeResult},
};

/// 0x50
pub fn pop(ctx: &mut ExecutionContext) -> OpcodeResult {
    let _ = ctx
        .machine_state
        .stack
        .pop()
        .ok_or(EVMError::EmptyStackError(ctx.clone()));

    Ok(())
}

/// 0x51
pub fn mload(ctx: &mut ExecutionContext) -> OpcodeResult {
    let offset = pop_n(ctx, 1)?[0];

    let offset: usize = offset
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(offset, ctx.clone()))?;

    let mut value_str = String::with_capacity(64);

    for i in 0..32 {
        let location = i + offset;
        let byte = ctx.machine_state.memory.get(location).unwrap_or(&0);
        value_str.push_str(&hex_string_from_byte(*byte))
    }

    let value = U256::from_str_radix(&value_str, 16)
        .map_err(|_| EVMError::FromStrRadixError(value_str, ctx.clone()))?;

    ctx.machine_state.stack.push(value);

    Ok(())
}

/// 0x52
pub fn mstore(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 2)?;

    let offset = stack_items[0];
    let offset: usize = offset
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(offset, ctx.clone()))?;

    let value = stack_items[1];

    for i in 0..=BYTES_IN_U256_FROM_ZERO {
        let byte = value.byte(BYTES_IN_U256_FROM_ZERO - i);
        if offset + i < ctx.machine_state.memory.len() {
            ctx.machine_state.memory[offset + i] = byte;
        } else {
            ctx.machine_state.memory.push(byte);
        }
    }

    Ok(())
}

/// 0x53
pub fn mstore8(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 2)?;

    let offset = stack_items[0];
    let offset: usize = offset
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(offset, ctx.clone()))?;

    let value = stack_items[1];
    let value: u8 = value
        .try_into()
        .map_err(|_| EVMError::U256ToU8Error(value, ctx.clone()))?;

    for i in 0..=BYTES_IN_U256_FROM_ZERO {
        if offset + i < ctx.machine_state.memory.len() {
            if i == BYTES_IN_U256_FROM_ZERO {
                ctx.machine_state.memory[offset + i] = value
            }
        } else {
            ctx.machine_state
                .memory
                .push(if i == BYTES_IN_U256_FROM_ZERO {
                    value
                } else {
                    0
                });
        }
    }

    Ok(())
}
