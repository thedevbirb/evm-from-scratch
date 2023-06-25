use crate::evm::utils::{
    constants::LOG_0,
    errors::EVMError,
    helpers::{pop_n, update_active_words_memory},
    types::{ExecutionContext, Log, OpcodeResult},
};

/// 0xa0 - 0xa4
pub fn log(ctx: &mut ExecutionContext) -> OpcodeResult {
    let opcode = ctx
        .input
        .bytecode
        .get(ctx.machine_state.pc)
        .ok_or(EVMError::NoBytecodeError(ctx.clone()))?;

    let n = opcode - LOG_0;

    let stack_items = pop_n(ctx, 2 + usize::from(n))?;
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

    ctx.accrued_substate.logs.push(Log {
        address: ctx.input.address,
        data,
        topics: if n > 0 {
            stack_items[3..].to_vec()
        } else {
            Vec::new()
        },
    });

    Ok(None)
}
