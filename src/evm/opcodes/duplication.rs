use crate::evm::utils::{
    constants::DUP_1,
    errors::EVMError,
    helpers::pop_n,
    types::{ExecutionContext, OpcodeResult},
};

/// 0x80 - 0x8f
pub fn dup(ctx: &mut ExecutionContext) -> OpcodeResult {
    let opcode = ctx
        .input
        .bytecode
        .get(ctx.machine_state.pc)
        .ok_or(EVMError::NoBytecodeError(ctx.clone()))?;
    let items_to_pop: usize = usize::from(1 + opcode - DUP_1);
    let stack_items = pop_n(ctx, items_to_pop)?;
    let item_to_dup = stack_items[items_to_pop - 1];

    stack_items
        .iter()
        .rev()
        .for_each(|s_i| ctx.machine_state.stack.push(*s_i));
    ctx.machine_state.stack.push(item_to_dup);

    Ok(None)
}
