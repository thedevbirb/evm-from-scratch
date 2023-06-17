use primitive_types::U256;

use crate::evm::utils::{
    errors::EVMError,
    helpers::pop_n_err,
    types::{ExecutionContext, OpcodeResult},
};

// 0x00
pub fn stop(_ctx: &mut ExecutionContext) -> OpcodeResult {
    Ok(())
}

// 0x01
pub fn add(ctx: &mut ExecutionContext) -> OpcodeResult {
    let values = pop_n_err(&mut ctx.machine_state.stack, 2)
        .map_err(|_| EVMError::EmptyStackError(ctx.clone()))?;

    let result = U256::overflowing_add(values[0], values[1]).0;

    ctx.machine_state.stack.push(result);

    Ok(())
}

// 0x02
pub fn mul(ctx: &mut ExecutionContext) -> OpcodeResult {
    let values = pop_n_err(&mut ctx.machine_state.stack, 2)
        .map_err(|_| EVMError::EmptyStackError(ctx.clone()))?;

    let result = U256::overflowing_mul(values[0], values[1]).0;

    ctx.machine_state.stack.push(result);

    Ok(())
}

// 0x03
pub fn sub(ctx: &mut ExecutionContext) -> OpcodeResult {
    let values = pop_n_err(&mut ctx.machine_state.stack, 2)
        .map_err(|_| EVMError::EmptyStackError(ctx.clone()))?;

    let result = U256::overflowing_sub(values[0], values[1]).0;

    ctx.machine_state.stack.push(result);

    Ok(())
}
