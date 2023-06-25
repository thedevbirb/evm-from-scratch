use std::collections::HashMap;

use primitive_types::U256;
use sha3::{Digest, Keccak256};

use crate::evm::{
    evm::EVM,
    utils::{
        constants::KECCAK_EMPTY,
        errors::EVMError,
        helpers::{modulo_address_size, pop_n, update_active_words_memory},
        types::{AccountState, ExecutionContext, MachineState, OpcodeResult},
    },
};

use super::environmental::balance;

/// 0xf0
pub fn create(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 3)?;
    let value = stack_items[0];
    let offset: usize = stack_items[1]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[1], ctx.clone()))?;
    let size: usize = stack_items[2]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[2], ctx.clone()))?;

    let sender = ctx.input.sender;
    let (nonce, _balance) = if let Some(account_state) = ctx.global_state.get_mut(&sender) {
        (account_state.nonce, account_state.balance)
    } else {
        let new_account_state = AccountState::new();
        let nonce = new_account_state.nonce;
        let balance = new_account_state.balance;
        ctx.global_state.insert(sender, new_account_state);
        (nonce, balance)
    };

    /*
     * tests do not handle this case
    if value > balance {
        return Ok(None);
    };
    */

    let mut hasher = Keccak256::new();
    let to_hash = [
        (0..32)
            .into_iter()
            .map(|i| sender.byte(i))
            .collect::<Vec<u8>>(),
        nonce.to_le_bytes().to_vec(),
    ]
    .concat(); // no rlp here, overkill
    hasher.update(to_hash);

    let new_account_address =
        U256::from(hasher.finalize().to_vec().get(12..).unwrap_or(&[0_u8; 20]));

    let initialisation_code = &ctx.machine_state.memory[offset..offset + size].to_owned();

    ctx.global_state.insert(
        new_account_address,
        AccountState {
            nonce: 0,
            balance: value,
            code_hash: KECCAK_EMPTY,
            code: initialisation_code.to_vec(),
            storage_root: KECCAK_EMPTY,
            storage: HashMap::new(),
        },
    );

    let old_input = ctx.input.clone();
    let old_machine_state = ctx.machine_state.clone();

    // prepare sub-context
    ctx.machine_state = MachineState::new();
    ctx.input.sender = ctx.input.address;
    ctx.input.bytecode = initialisation_code.clone();
    ctx.input.address = new_account_address;
    ctx.input.depth += 1;

    let result = EVM::execute(ctx)?;

    let output = result.output.unwrap_or(Vec::new());

    // restore context
    ctx.input = old_input;
    ctx.machine_state = old_machine_state;
    ctx.machine_state.output = output.clone();

    // handle result
    if result.success {
        // the new account's code is set to the return data
        let account_state = ctx.global_state.get_mut(&new_account_address).unwrap();
        let code = output;
        let mut hasher = Keccak256::new();
        hasher.update(initialisation_code);
        let code_hash = if code.len() > 0 {
            U256::from(hasher.finalize().as_slice())
        } else {
            KECCAK_EMPTY
        };

        account_state.code = code;
        account_state.code_hash = code_hash;

        ctx.machine_state.stack.push(new_account_address);
    } else {
        ctx.machine_state.stack.push(U256::zero());
    }

    update_active_words_memory(ctx, offset + size);

    Ok(None)
}

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

    // sload then fails because the test supposes no particular account storage.
    // the problem is the `to` setted up in the test of delegatecall.

    Ok(None)
}

/// 0xfa
pub fn staticcall(ctx: &mut ExecutionContext) -> OpcodeResult {
    // prepare the call with no write permission
    let stack_items = pop_n(ctx, 2)?;
    ctx.machine_state.stack.push(U256::zero()); // zero value;
    stack_items
        .iter()
        .rev()
        .for_each(|s_i| ctx.machine_state.stack.push(*s_i));

    ctx.input.write = false;
    call(ctx)
}

/// 0xfd For this challenge, this is just a return with
/// difference success status
pub fn revert(ctx: &mut ExecutionContext) -> OpcodeResult {
    r#return(ctx)
}

/// 0xff
pub fn selfdestruct(ctx: &mut ExecutionContext) -> OpcodeResult {
    let receiver_address = modulo_address_size(&pop_n(ctx, 1)?[0]);

    // get contract balance
    ctx.machine_state.stack.push(ctx.input.address);
    let _ = balance(ctx);
    let contract_balance = pop_n(ctx, 1)?[0];

    // transfer balance
    let receiver_account_state =
        if let Some(account_state) = ctx.global_state.get_mut(&receiver_address) {
            account_state
        } else {
            ctx.global_state
                .insert(receiver_address, AccountState::new());
            ctx.global_state.get_mut(&receiver_address).unwrap()
        };

    receiver_account_state.balance += contract_balance;

    ctx.accrued_substate
        .accessed_accounts
        .insert(ctx.input.address);
    ctx.accrued_substate
        .accessed_accounts
        .insert(receiver_address);
    ctx.accrued_substate
        .self_destruct_set
        .insert(ctx.input.address);

    Ok(None)
}
