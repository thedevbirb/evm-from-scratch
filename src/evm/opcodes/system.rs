use crate::evm::utils::{types::{ExecutionContext, OpcodeResult}, errors::EVMError, helpers::pop_n};

/// 0xf3
pub fn r#return(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 2)?;
    let offset: usize = stack_items[0]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[0], ctx.clone()))?;
    let size: usize = stack_items[1]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[1], ctx.clone()))?;

    Ok(())

}
