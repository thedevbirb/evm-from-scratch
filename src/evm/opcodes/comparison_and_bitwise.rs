use primitive_types::U256;

use crate::evm::utils::{
    errors::EVMError,
    helpers::{convert_twos_complement, is_negative, pop_n, swap_1},
    traits::Bool,
    types::{ExecutionContext, OpcodeResult}, constants::BYTES_IN_U256_FROM_ZERO,
};

/// 0x10
pub fn lt(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 2)?;
    ctx.machine_state
        .stack
        .push(U256::from_bool(stack_items[0] < stack_items[1]));

    Ok(None)
}

/// 0x11
pub fn gt(ctx: &mut ExecutionContext) -> OpcodeResult {
    let _ = swap_1(ctx);
    lt(ctx)
}

/// 0x12
pub fn slt(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 2)?;
    let a = stack_items[0];
    let b = stack_items[1];

    match (is_negative(&a), is_negative(&b)) {
        (false, true) => ctx.machine_state.stack.push(U256::zero()),
        (true, false) => ctx.machine_state.stack.push(U256::one()),
        _ => ctx
            .machine_state
            .stack
            .push(U256::from_bool(stack_items[0] < stack_items[1])),
    }

    Ok(None)
}

/// 0x13
pub fn sgt(ctx: &mut ExecutionContext) -> OpcodeResult {
    let _ = swap_1(ctx);
    slt(ctx)
}

/// 0x14
pub fn eq(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 2)?;
    ctx.machine_state
        .stack
        .push(U256::from_bool(stack_items[0] == stack_items[1]));

    Ok(None)
}

/// 0x15
pub fn iszero(ctx: &mut ExecutionContext) -> OpcodeResult {
    let value = pop_n(ctx, 1)?[0];
    ctx.machine_state
        .stack
        .push(U256::from_bool(value == U256::zero()));

    Ok(None)
}

/// 0x16
pub fn and(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 2)?;
    ctx.machine_state
        .stack
        .push(stack_items[0] & stack_items[1]);

    Ok(None)
}

/// 0x17
pub fn or(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 2)?;
    ctx.machine_state
        .stack
        .push(stack_items[0] | stack_items[1]);

    Ok(None)
}

/// 0x18
pub fn xor(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 2)?;
    ctx.machine_state
        .stack
        .push(stack_items[0] ^ stack_items[1]);

    Ok(None)
}

/// 0x19
pub fn not(ctx: &mut ExecutionContext) -> OpcodeResult {
    let value = pop_n(ctx, 1)?[0];
    ctx.machine_state.stack.push(!value);

    Ok(None)
}

/// 0x1a
pub fn byte(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 2)?;
    let index: usize = stack_items[0]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[0], ctx.clone()))?;
    let value = stack_items[1];

    let result = if index > BYTES_IN_U256_FROM_ZERO {
        U256::zero()
    } else {
        U256::from(value.byte(BYTES_IN_U256_FROM_ZERO - index))
    };

    ctx.machine_state
        .stack
        .push(result);

    Ok(None)
}

/// 0x1b
pub fn shl(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 2)?;
    let shift = stack_items[0];
    let value = stack_items[1];

    ctx.machine_state.stack.push(value << shift);

    Ok(None)
}

/// 0x1c
pub fn shr(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 2)?;
    let shift = stack_items[0];
    let value = stack_items[1];

    ctx.machine_state.stack.push(value >> shift);

    Ok(None)
}

/// 0x1d
pub fn sar(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 2)?;
    let shift = stack_items[0];
    let mut value = stack_items[1];

    let is_negative = is_negative(&value);

    if is_negative {
        value = convert_twos_complement(value);
    }

    if shift >= U256::from(32) {
        value = value & U256::zero();
    } else {
        value = value >> shift;
    }

    if is_negative {
        if value == U256::zero() {
            value = U256::MAX;
        } else {
            value = convert_twos_complement(value);
        }
    }
    ctx.machine_state.stack.push(value);

    Ok(None)
}
