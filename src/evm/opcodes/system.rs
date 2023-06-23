use crate::evm::{utils::{
    errors::EVMError,
    helpers::{pop_n, update_active_words_memory},
    types::{ExecutionContext, OpcodeResult},
}, evm::EVM};

/// 0xf1
pub fn call(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 7)?;

    let _gas = stack_items[0];
    let address = stack_items[1];
    let value = stack_items[2];
    let args_offset: usize = stack_items[3]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[3], ctx.clone()))?;
    let args_size: usize = stack_items[4]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[4], ctx.clone()))?;
    let ret_offset: usize = stack_items[5]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[5], ctx.clone()))?;
    let ret_size: usize = stack_items[6]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[6], ctx.clone()))?;

    let mut calldata: Vec<u8> = Vec::new();
    for i in args_offset..args_offset + args_size {
        calldata.push(*ctx.machine_state.memory.get(i).unwrap_or(&0))
    }
    update_active_words_memory(ctx, args_offset+args_size);

    let old_input = ctx.input.clone();
    let old_machine_state = ctx.machine_state.clone();

    ctx.input.address = address;
    ctx.input.value = value;
    ctx.input.bytecode = if let Some(account_state) = ctx.global_state.get(&address) {
        account_state.code.clone()
    } else {
        Vec::new()
    };


    let result = EVM::execute(ctx);

    ctx.input = old_input;
    ctx.machine_state = old_machine_state;

    Ok(None)
}

/// 0xf3
pub fn r#return(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 2)?;
    let offset: usize = stack_items[0]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[0], ctx.clone()))?;
    let size: usize = stack_items[1]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[1], ctx.clone()))?;

    let mut data: Vec<u8> = Vec::with_capacity(size);
    for i in offset..offset + size {
        data.push(*ctx.machine_state.memory.get(i).unwrap_or(&0))
    }

    update_active_words_memory(ctx, offset + size);

    Ok(Some(data))
}

/// 0xfd For this challenge, this is just a return with
/// difference success status
pub fn revert(ctx: &mut ExecutionContext) -> OpcodeResult {
    r#return(ctx)
}
