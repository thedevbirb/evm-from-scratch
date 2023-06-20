use primitive_types::U256;

use crate::evm::utils::{
    errors::EVMError,
    helpers::{hex_string_from_bytes, pop_n, update_active_words_memory},
    types::{ExecutionContext, OpcodeResult},
};

/// 0x30
pub fn address(ctx: &mut ExecutionContext) -> OpcodeResult {
    ctx.machine_state.stack.push(ctx.input.address);
    Ok(())
}

/// 0x31
pub fn balance(ctx: &mut ExecutionContext) -> OpcodeResult {
    let address = pop_n(ctx, 1)?[0];
    let balance = if let Some(account_state) = ctx.global_state.get(&address) {
        account_state.balance
    } else {
        U256::zero()
    };

    ctx.machine_state.stack.push(balance);

    Ok(())
}

/// 0x32
pub fn origin(ctx: &mut ExecutionContext) -> OpcodeResult {
    ctx.machine_state.stack.push(ctx.input.origin);

    Ok(())
}

/// 0x33 Solidity calls this msg.sender
pub fn caller(ctx: &mut ExecutionContext) -> OpcodeResult {
    ctx.machine_state.stack.push(ctx.input.sender);

    Ok(())
}

/// 0x34 Solidity calls this msg.value
pub fn callvalue(ctx: &mut ExecutionContext) -> OpcodeResult {
    ctx.machine_state.stack.push(ctx.input.value);
    Ok(())
}

/// 0x35
pub fn calldataload(ctx: &mut ExecutionContext) -> OpcodeResult {
    let offset = pop_n(ctx, 1)?[0];
    let offset: usize = offset
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(offset, ctx.clone()))?;

    let data: Vec<u8> = vec![0_u8; 32]
        .iter()
        .enumerate()
        .map(|(idx, zero_byte)| *ctx.input.data.get(offset + idx).unwrap_or(zero_byte))
        .collect();

    let str_data = hex_string_from_bytes(&data);
    let data = U256::from_str_radix(&str_data, 16)
        .map_err(|_| EVMError::FromStrRadixError(str_data, ctx.clone()))?;

    ctx.machine_state.stack.push(data);

    Ok(())
}

/// 0x36
pub fn calldatasize(ctx: &mut ExecutionContext) -> OpcodeResult {
    let size = U256::from(ctx.input.data.len());

    ctx.machine_state.stack.push(size);

    Ok(())
}

/// 0x37
pub fn calldatacopy(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 3)?;

    let dest_offset: usize = stack_items[0]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[0], ctx.clone()))?;
    let offset: usize = stack_items[1]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[1], ctx.clone()))?;
    let size: usize = stack_items[2]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[2], ctx.clone()))?;

    vec![0_u8; size]
        .iter()
        .enumerate()
        .for_each(|(idx, zero_byte)| {
            let byte = ctx.input.data.get(offset + idx).unwrap_or(zero_byte);
            ctx.machine_state.memory[dest_offset + idx] = *byte
        });

    update_active_words_memory(ctx, offset + size);

    Ok(())
}

/// 0x3a
pub fn gasprice(ctx: &mut ExecutionContext) -> OpcodeResult {
    ctx.machine_state.stack.push(ctx.input.price);
    Ok(())
}
