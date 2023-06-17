use crate::evm::utils::{
    errors::EVMError,
    types::{ExecutionContext, OpcodeResult},
};

// 0x50
pub fn pop(ctx: &mut ExecutionContext) -> OpcodeResult {
    let _ = ctx
        .machine_state
        .stack
        .pop()
        .ok_or(EVMError::EmptyStackError(ctx.clone()));
    Ok(())
}
