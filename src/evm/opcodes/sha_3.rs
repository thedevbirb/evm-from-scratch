use primitive_types::U256;
use sha3::{Digest, Keccak256};

use crate::evm::utils::{
    errors::EVMError,
    helpers::{pop_n, update_active_words_memory},
    types::{ExecutionContext, OpcodeResult},
};

pub fn sha3(ctx: &mut ExecutionContext) -> OpcodeResult {
    let mut hasher = Keccak256::new();

    let stack_items = pop_n(ctx, 2)?;

    let starting_offset: usize = stack_items[0]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[0], ctx.clone()))?;
    let ending_offset: usize = stack_items[1]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[1], ctx.clone()))?;

    let data = &ctx.machine_state.memory[starting_offset..ending_offset];
    hasher.update(data);

    let hash = hasher.finalize();
    let hash_vec = hash.to_vec();

    let val = U256::from(&hash_vec[..]);
    ctx.machine_state.stack.push(val);

    update_active_words_memory(ctx, ending_offset);

    Ok(None)
}
