use primitive_types::U256;

use crate::evm::utils::{
    constants::BYTES_IN_U256_FROM_ZERO,
    errors::EVMError,
    helpers::{hex_string_from_byte, pop_n, update_active_words_memory},
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

    println!("{:x}", offset);

    let mut value_str = String::with_capacity(64);

    for i in 0..=BYTES_IN_U256_FROM_ZERO {
        let location = i + offset;
        let byte = ctx.machine_state.memory.get(location).unwrap_or(&0);
        value_str.push_str(&hex_string_from_byte(*byte))
    }

    let value = U256::from_str_radix(&value_str, 16)
        .map_err(|_| EVMError::FromStrRadixError(value_str, ctx.clone()))?;

    ctx.machine_state.stack.push(value);

    println!("{:x}", offset + BYTES_IN_U256_FROM_ZERO);

    update_active_words_memory(ctx, offset + BYTES_IN_U256_FROM_ZERO);

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

    update_active_words_memory(ctx, offset + BYTES_IN_U256_FROM_ZERO);

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

    ctx.machine_state.memory[offset] = value;

    update_active_words_memory(ctx, offset);

    Ok(())
}

/// 0x59
pub fn msize(ctx: &mut ExecutionContext) -> OpcodeResult {
    let msize = U256::from(ctx.machine_state.active_words_memory * 32);
    ctx.machine_state.stack.push(msize);
    Ok(())
}
