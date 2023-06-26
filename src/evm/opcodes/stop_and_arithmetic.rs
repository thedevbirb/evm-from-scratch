use std::ops::Div;

use primitive_types::U256;

use crate::evm::utils::{
    helpers::pop_n,
    types::{ExecutionContext, OpcodeResult},
};

/// 0x00
pub fn stop(_ctx: &mut ExecutionContext) -> OpcodeResult {
    Ok(None)
}

/// 0x01
pub fn add(ctx: &mut ExecutionContext) -> OpcodeResult {
    let values = pop_n(ctx, 2)?;

    let result = U256::overflowing_add(values[0], values[1]).0;
    ctx.machine_state.stack.push(result);

    Ok(None)
}

/// 0x02
pub fn mul(ctx: &mut ExecutionContext) -> OpcodeResult {
    let values = pop_n(ctx, 2)?;

    let result = U256::overflowing_mul(values[0], values[1]).0;
    ctx.machine_state.stack.push(result);

    Ok(None)
}

/// 0x03
pub fn sub(ctx: &mut ExecutionContext) -> OpcodeResult {
    let values = pop_n(ctx, 2)?;

    let result = U256::overflowing_sub(values[0], values[1]).0;

    ctx.machine_state.stack.push(result);

    Ok(None)
}

/// 0x04
pub fn div(ctx: &mut ExecutionContext) -> OpcodeResult {
    let values = pop_n(ctx, 2)?;

    let result = values[0].checked_div(values[1]).unwrap_or(U256::zero());
    ctx.machine_state.stack.push(result);

    Ok(None)
}

/// 0x06
pub fn r#mod(ctx: &mut ExecutionContext) -> OpcodeResult {
    let values = pop_n(ctx, 2)?;

    let result = values[0].checked_rem(values[1]).unwrap_or(U256::zero());
    ctx.machine_state.stack.push(result);

    Ok(None)
}

/// 0x07
pub fn addmod(ctx: &mut ExecutionContext) -> OpcodeResult {
    let _ = add(ctx);
    r#mod(ctx)
}

/// 0x09
pub fn mulmod(ctx: &mut ExecutionContext) -> OpcodeResult {
    let values = pop_n(ctx, 3)?;

    let res_mul = values[0].full_mul(values[1]);
    let res_modulo = res_mul.checked_rem(values[2].into());

    ctx.machine_state
        .stack
        .push(if let Some(result) = res_modulo {
            result.try_into().unwrap_or(U256::zero())
        } else {
            U256::zero()
        });

    Ok(None)
}

/// 0x0a
pub fn exp(ctx: &mut ExecutionContext) -> OpcodeResult {
    let values = pop_n(ctx, 2)?;

    let result = values[0].overflowing_pow(values[1]).0;
    ctx.machine_state.stack.push(result);

    Ok(None)
}

/// 0x0b
pub fn signextend(ctx: &mut ExecutionContext) -> OpcodeResult {
    let values = pop_n(ctx, 2)?;

    let result = values[0].overflowing_pow(values[1]).0;
    ctx.machine_state.stack.push(result);

    Ok(None)
}
