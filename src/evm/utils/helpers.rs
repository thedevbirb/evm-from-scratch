use std::collections::HashMap;

use primitive_types::U256;

use crate::evm::opcodes;

use super::{
    errors::EVMError,
    types::{Address, ExecutionContext, GlobalState, Opcodes},
};

/// Models the EMPTY function in the yellow paper
pub fn is_account_empty(state: GlobalState, address: Address) -> bool {
    if let Some(account_state) = state.get(&address) {
        account_state.nonce == 0 && account_state.balance.is_zero()
    } else {
        false
    }
}

/// Models the DEAD function in the yellow paper
pub fn is_account_dead(state: GlobalState, address: Address) -> bool {
    state.get(&address).is_none() || is_account_empty(state, address)
}

/// Returns a vector of length `n` in which all elements are indeed present
pub fn pop_n(ctx: &mut ExecutionContext, n: usize) -> Result<Vec<U256>, EVMError> {
    if ctx.machine_state.stack.len() < n {
        return Err(EVMError::EmptyStackError(ctx.clone()));
    }

    let mut result = Vec::with_capacity(n);

    (0..n).for_each(|_| {
        if let Some(v) = ctx.machine_state.stack.pop() {
            result.push(v);
        }
    });

    Ok(result)
}

/// Convert from hex string of even length to a vector of bytes
/// The `reverse` option adds the last significant byte in the first position
pub fn bytes_from_hex_str(str: &str, reverse: bool) -> Result<Vec<u8>, ()> {
    if reverse {
        todo!();
    }

    let is_0x_based = if let Some(s) = str.get(0..2) {
        s == "0x"
    } else {
        false
    };

    let bytes_count = str.len() / 2;
    let mut vec: Vec<u8> = Vec::with_capacity(bytes_count);

    let mut i = if is_0x_based { 2 } else { 0 };

    while let Some(byte_str) = str.get(i..=i + 1) {
        let byte = u8::from_str_radix(byte_str, 16).or_else(|_| Err(()))?;
        vec.push(byte);
        i += 2;
    }

    Ok(vec)
}

/// Convert bytes array to hex string, with each byte mapped to two chars
pub fn hex_string_from_bytes(vec: &[u8]) -> String {
    let mut str = String::with_capacity(vec.len() * 2);

    vec.iter()
        .for_each(|byte| str.push_str(&hex_string_from_byte(*byte)));

    str
}

/// Convert byte to hex string made of two chars
pub fn hex_string_from_byte(byte: u8) -> String {
    if byte < 16 {
        format!("0{:x}", byte)
    } else {
        format!("{:x}", byte)
    }
}

/// idx is in bytes
pub fn update_active_words_memory(ctx: &mut ExecutionContext, last_accessed_memory_idx: usize) {
    let last_accessed_word_idx = last_accessed_memory_idx / 32;
    ctx.machine_state.active_words_memory = usize::max(
        ctx.machine_state.active_words_memory,
        last_accessed_word_idx + 1,
    );
}

pub fn get_opcodes() -> Opcodes {
    let mut opcodes: Opcodes = HashMap::new();

    opcodes.insert(0x00, Box::new(opcodes::stop_and_arithmetic::stop));
    opcodes.insert(0x01, Box::new(opcodes::stop_and_arithmetic::add));
    opcodes.insert(0x02, Box::new(opcodes::stop_and_arithmetic::mul));
    opcodes.insert(0x03, Box::new(opcodes::stop_and_arithmetic::sub));
    //    opcodes.insert(0x04, Box::new(opcodes::arithmetic::div));
    //    opcodes.insert(0x05, Box::new(opcodes::arithmetic::s_div));
    //    opcodes.insert(0x06, Box::new(opcodes::arithmetic::modulo));
    //    opcodes.insert(0x07, Box::new(opcodes::arithmetic::s_modulo));
    //    opcodes.insert(0x08, Box::new(opcodes::arithmetic::add_mod));
    //    opcodes.insert(0x09, Box::new(opcodes::arithmetic::mul_mod));
    //    opcodes.insert(0x0a, Box::new(opcodes::arithmetic::exp));
    //
    //    opcodes.insert(0x10, Box::new(opcodes::logic::lt));
    //    opcodes.insert(0x11, Box::new(opcodes::logic::gt));
    //    opcodes.insert(0x12, Box::new(opcodes::logic::slt));
    //    opcodes.insert(0x13, Box::new(opcodes::logic::sgt));
    //    opcodes.insert(0x14, Box::new(opcodes::logic::eq));
    //    opcodes.insert(0x15, Box::new(opcodes::logic::is_zero));
    //    opcodes.insert(0x16, Box::new(opcodes::logic::and));
    //    opcodes.insert(0x17, Box::new(opcodes::logic::or));
    //    opcodes.insert(0x18, Box::new(opcodes::logic::xor));
    //    opcodes.insert(0x19, Box::new(opcodes::logic::not));
    //
    //    opcodes.insert(0x1b, Box::new(opcodes::misc::shl));
    //    opcodes.insert(0x1c, Box::new(opcodes::misc::shr));
    //    opcodes.insert(0x1d, Box::new(opcodes::misc::sar));
    //    opcodes.insert(0x1a, Box::new(opcodes::misc::byte));
    opcodes.insert(0x20, Box::new(opcodes::sha_3::sha3));
    //
    opcodes.insert(0x30, Box::new(opcodes::environmental::address));
    //    opcodes.insert(0x31, Box::new(opcodes::state::balance));
    opcodes.insert(0x32, Box::new(opcodes::environmental::origin));
    opcodes.insert(0x33, Box::new(opcodes::environmental::caller));
    //    opcodes.insert(0x34, Box::new(opcodes::transaction::callvalue));
    //    opcodes.insert(0x35, Box::new(opcodes::transaction::calldataload));
    //    opcodes.insert(0x36, Box::new(opcodes::transaction::calldatasize));
    //    opcodes.insert(0x37, Box::new(opcodes::transaction::calldatacopy));
    //    opcodes.insert(0x38, Box::new(opcodes::misc::codesize));
    //    opcodes.insert(0x39, Box::new(opcodes::misc::codecopy));
    opcodes.insert(0x3a, Box::new(opcodes::environmental::gasprice));
    //    opcodes.insert(0x3b, Box::new(opcodes::environmental::extcodesize));
    //    opcodes.insert(0x3c, Box::new(opcodes::environmental::extcodecopy));
    //    opcodes.insert(0x3f, Box::new(opcodes::environmental::extcodehash));
    //
    //    opcodes.insert(0x40, Box::new(opcodes::block::blockhash));
    //    opcodes.insert(0x41, Box::new(opcodes::block::coinbase));
    //    opcodes.insert(0x42, Box::new(opcodes::block::timestamp));
    //    opcodes.insert(0x43, Box::new(opcodes::block::number));
    //    opcodes.insert(0x44, Box::new(opcodes::block::difficulty));
    //    opcodes.insert(0x45, Box::new(opcodes::block::gaslimit));
    //    opcodes.insert(0x46, Box::new(opcodes::block::chain));
    //    opcodes.insert(0x47, Box::new(opcodes::block::selfbalance));
    //    opcodes.insert(0x48, Box::new(opcodes::block::basefee));
    //
    //    // opcodes.insert(0x0b, Box::new(opcodes::sign_extend));
    opcodes.insert(0x50, Box::new(opcodes::stack_memory_storage_flow::pop));
    opcodes.insert(0x51, Box::new(opcodes::stack_memory_storage_flow::mload));
    opcodes.insert(0x52, Box::new(opcodes::stack_memory_storage_flow::mstore));
    opcodes.insert(0x53, Box::new(opcodes::stack_memory_storage_flow::mstore8));
    //    opcodes.insert(0x54, Box::new(opcodes::storage::sload));
    //    opcodes.insert(0x55, Box::new(opcodes::storage::sstore));
    //    opcodes.insert(0x56, Box::new(opcodes::stack::jump));
    //    opcodes.insert(0x57, Box::new(opcodes::stack::jumpi));
    //    opcodes.insert(0x58, Box::new(opcodes::stack::pc));
    opcodes.insert(0x59, Box::new(opcodes::stack_memory_storage_flow::msize));
    //    opcodes.insert(0x5a, Box::new(opcodes::misc::gas));
    //    opcodes.insert(0x5b, Box::new(opcodes::stack::jumpdest));
    //

    opcodes.insert(0x5f, Box::new(opcodes::push::push));
    opcodes.insert(0x60, Box::new(opcodes::push::push));
    opcodes.insert(0x61, Box::new(opcodes::push::push));
    opcodes.insert(0x62, Box::new(opcodes::push::push));
    opcodes.insert(0x63, Box::new(opcodes::push::push));
    opcodes.insert(0x64, Box::new(opcodes::push::push));
    opcodes.insert(0x65, Box::new(opcodes::push::push));
    opcodes.insert(0x66, Box::new(opcodes::push::push));
    opcodes.insert(0x67, Box::new(opcodes::push::push));
    opcodes.insert(0x68, Box::new(opcodes::push::push));
    opcodes.insert(0x69, Box::new(opcodes::push::push));
    opcodes.insert(0x6a, Box::new(opcodes::push::push));
    opcodes.insert(0x6b, Box::new(opcodes::push::push));
    opcodes.insert(0x6c, Box::new(opcodes::push::push));
    opcodes.insert(0x6d, Box::new(opcodes::push::push));
    opcodes.insert(0x6e, Box::new(opcodes::push::push));
    opcodes.insert(0x6f, Box::new(opcodes::push::push));
    opcodes.insert(0x70, Box::new(opcodes::push::push));
    opcodes.insert(0x71, Box::new(opcodes::push::push));
    opcodes.insert(0x72, Box::new(opcodes::push::push));
    opcodes.insert(0x73, Box::new(opcodes::push::push));
    opcodes.insert(0x74, Box::new(opcodes::push::push));
    opcodes.insert(0x75, Box::new(opcodes::push::push));
    opcodes.insert(0x76, Box::new(opcodes::push::push));
    opcodes.insert(0x77, Box::new(opcodes::push::push));
    opcodes.insert(0x78, Box::new(opcodes::push::push));
    opcodes.insert(0x79, Box::new(opcodes::push::push));
    opcodes.insert(0x7a, Box::new(opcodes::push::push));
    opcodes.insert(0x7b, Box::new(opcodes::push::push));
    opcodes.insert(0x7c, Box::new(opcodes::push::push));
    opcodes.insert(0x7d, Box::new(opcodes::push::push));
    opcodes.insert(0x7e, Box::new(opcodes::push::push));
    opcodes.insert(0x7f, Box::new(opcodes::push::push));
    //    insert_push_n_functions(&mut opcodes);
    //    insert_dup_n_functions(&mut opcodes);
    //    insert_swap_n_functions(&mut opcodes);
    //    insert_log_n_function(&mut opcodes);
    //
    //    opcodes.insert(0xf1, Box::new(opcodes::system::return_data));
    //    opcodes.insert(0xf3, Box::new(opcodes::system::return_data));
    //    opcodes.insert(0xfe, Box::new(opcodes::misc::invalid));
    //    opcodes.insert(0xfd, Box::new(opcodes::system::revert));

    opcodes
}
