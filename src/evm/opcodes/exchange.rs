use crate::evm::utils::{
    constants::SWAP_1,
    errors::EVMError,
    helpers::pop_n,
    types::{ExecutionContext, OpcodeResult},
};

/// 0x90 - 0x9f
pub fn swap(ctx: &mut ExecutionContext) -> OpcodeResult {
    let opcode = ctx
        .input
        .bytecode
        .get(ctx.machine_state.pc)
        .ok_or(EVMError::NoBytecodeError(ctx.clone()))?;

    let n: usize = (opcode - SWAP_1 + 1).into();

    let stack_items = pop_n(ctx, (n + 1).into())?;
    let first = stack_items[0];
    let last = stack_items[n];

    ctx.machine_state.stack.push(first);
    stack_items[1..n]
        .iter()
        .rev()
        .for_each(|s_i| ctx.machine_state.stack.push(*s_i));
    ctx.machine_state.stack.push(last);

    Ok(None)
}
