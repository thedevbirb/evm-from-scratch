use primitive_types::U256;

use crate::evm::{
    evm::EVM,
    utils::{
        errors::EVMError,
        helpers::{modulo_address_size, pop_n, update_active_words_memory},
        types::{AccountState, ExecutionContext, MachineState, OpcodeResult},
    },
};

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

    let insufficient_balance = ctx
        .global_state
        .get(&ctx.input.address)
        .unwrap_or(&AccountState::new()) // maybe an error for this?
        .balance
        < value;
    let call_depth_limit_reached = ctx.input.depth == 1024;

    if insufficient_balance || call_depth_limit_reached {
        ctx.machine_state.stack.push(U256::zero());
        return Ok(None);
    }

    // prepare call
    let mut calldata: Vec<u8> = Vec::new();
    for i in args_offset..args_offset + args_size {
        calldata.push(*ctx.machine_state.memory.get(i).unwrap_or(&0))
    }

    update_active_words_memory(ctx, args_offset + args_size);

    let old_input = ctx.input.clone();
    let old_machine_state = ctx.machine_state.clone();

    ctx.input.sender = old_input.address;
    ctx.input.address = modulo_address_size(&address);
    ctx.input.value = value;
    ctx.input.bytecode = if let Some(account_state) = ctx.global_state.get(&address) {
        account_state.code.clone()
    } else {
        Vec::new()
    };
    ctx.input.depth += 1;
    ctx.machine_state = MachineState::new();

    // call
    let result = EVM::execute(ctx)?;

    // restore current ctx and manage result
    ctx.input = old_input;
    ctx.machine_state = old_machine_state;
    ctx.machine_state.output = result.output.unwrap_or(Vec::new());
    ctx.machine_state.stack.push(U256::from(if result.success {
        U256::from(1)
    } else {
        U256::zero()
    }));
    ctx.accrued_substate.accessed_accounts.insert(address);

    let return_data_length = ret_size.min(ctx.machine_state.output.len());

    for i in 0..return_data_length {
        ctx.machine_state.memory[ret_offset + i] = *ctx.machine_state.output.get(i).unwrap_or(&0);
    }
    update_active_words_memory(ctx, ret_offset + ret_size);

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

/// 0xf4
pub fn delegatecall(ctx: &mut ExecutionContext) -> OpcodeResult {
    dbg!(&ctx.machine_state.stack);
    let stack_items = pop_n(ctx, 6)?;

    let _gas = stack_items[0];
    let address = stack_items[1];
    let args_offset: usize = stack_items[2]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[2], ctx.clone()))?;
    let args_size: usize = stack_items[3]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[3], ctx.clone()))?;
    let ret_offset: usize = stack_items[4]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[4], ctx.clone()))?;
    let ret_size: usize = stack_items[5]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[5], ctx.clone()))?;

    let insufficient_balance = ctx
        .global_state
        .get(&ctx.input.address)
        .unwrap_or(&AccountState::new()) // maybe an error for this?
        .balance
        < ctx.input.value;
    let call_depth_limit_reached = ctx.input.depth == 1024;

    if insufficient_balance || call_depth_limit_reached {
        ctx.machine_state.stack.push(U256::zero());
        return Ok(None);
    }

    // prepare call
    let mut calldata: Vec<u8> = Vec::new();
    for i in args_offset..args_offset + args_size {
        calldata.push(*ctx.machine_state.memory.get(i).unwrap_or(&0))
    }

    update_active_words_memory(ctx, args_offset + args_size);

    let old_input = ctx.input.clone();
    let old_machine_state = ctx.machine_state.clone();

    ctx.input.address = modulo_address_size(&address);
    ctx.input.bytecode = if let Some(account_state) = ctx.global_state.get(&address) {
        account_state.code.clone()
    } else {
        Vec::new()
    };
    ctx.input.depth += 1;
    ctx.machine_state = MachineState::new();

    // call
    let result = EVM::execute(ctx)?;

    // restore current ctx and manage result
    ctx.input = old_input;
    ctx.machine_state = old_machine_state;
    ctx.machine_state.output = result.output.unwrap_or(Vec::new());
    ctx.machine_state.stack.push(U256::from(if result.success {
        U256::from(1)
    } else {
        U256::zero()
    }));
    ctx.accrued_substate.accessed_accounts.insert(address);

    let return_data_length = ret_size.min(ctx.machine_state.output.len());

    for i in 0..return_data_length {
        ctx.machine_state.memory[ret_offset + i] = *ctx.machine_state.output.get(i).unwrap_or(&0);
    }
    update_active_words_memory(ctx, ret_offset + ret_size);

    Ok(None)
}

/// 0xfd For this challenge, this is just a return with
/// difference success status
pub fn revert(ctx: &mut ExecutionContext) -> OpcodeResult {
    r#return(ctx)
}

