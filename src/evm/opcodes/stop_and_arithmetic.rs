use primitive_types::U256;

use crate::evm::utils::{
    errors::EVMError,
    helpers::{convert_twos_complement, is_negative, pop_n},
    types::{ExecutionContext, OpcodeResult},
};

/// 0x00
pub fn stop(_ctx: &mut ExecutionContext) -> OpcodeResult {
    Ok(None)
}

/// 0x01
pub fn add(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 2)?;

    let result = U256::overflowing_add(stack_items[0], stack_items[1]).0;
    ctx.machine_state.stack.push(result);

    Ok(None)
}

/// 0x02
pub fn mul(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 2)?;

    let result = U256::overflowing_mul(stack_items[0], stack_items[1]).0;
    ctx.machine_state.stack.push(result);

    Ok(None)
}

/// 0x03
pub fn sub(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 2)?;

    let result = U256::overflowing_sub(stack_items[0], stack_items[1]).0;

    ctx.machine_state.stack.push(result);

    Ok(None)
}

/// 0x04
pub fn div(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 2)?;

    let result = stack_items[0]
        .checked_div(stack_items[1])
        .unwrap_or(U256::zero());
    ctx.machine_state.stack.push(result);

    Ok(None)
}

/// 0x05
pub fn sdiv(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 2)?;

    let mut a = stack_items[0];
    let mut b = stack_items[1];

    if b == U256::zero() {
        ctx.machine_state.stack.push(U256::zero());
        return Ok(None);
    }

    let (is_a_negative, is_b_negative) = (is_negative(&a), is_negative(&b));

    if is_a_negative {
        a = convert_twos_complement(a);
    };
    if is_b_negative {
        b = convert_twos_complement(b);
    };

    let mut result = a.checked_div(b).unwrap();
    if is_a_negative ^ is_b_negative {
        result = convert_twos_complement(result)
    }

    ctx.machine_state.stack.push(result);

    Ok(None)
}

/// 0x06
pub fn r#mod(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 2)?;

    let result = stack_items[0]
        .checked_rem(stack_items[1])
        .unwrap_or(U256::zero());
    ctx.machine_state.stack.push(result);

    Ok(None)
}

/// 0x07
pub fn smod(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 2)?;

    let mut a = stack_items[0];
    let mut n = stack_items[1];

    if n == U256::zero() {
        ctx.machine_state.stack.push(U256::zero());
        return Ok(None);
    }

    let is_a_negative = is_negative(&a);
    let is_n_negative = is_negative(&n);

    // Recall that $$ka \equiv kb (\mod n)$$ for any integer $k$
    if is_a_negative {
        a = convert_twos_complement(a);
    }
    if is_n_negative {
        n = convert_twos_complement(n);
    }

    if !is_a_negative && !is_n_negative {
        ctx.machine_state.stack.push(a % n);
    } else {
        ctx.machine_state.stack.push(convert_twos_complement(a % n))
    }

    Ok(None)
}

/// 0x08
pub fn addmod(ctx: &mut ExecutionContext) -> OpcodeResult {
    let _ = add(ctx);
    r#mod(ctx)
}

/// 0x09
pub fn mulmod(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 3)?;

    let res_mul = stack_items[0].full_mul(stack_items[1]);
    let res_modulo = res_mul.checked_rem(stack_items[2].into());

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
    let stack_items = pop_n(ctx, 2)?;

    let result = stack_items[0].overflowing_pow(stack_items[1]).0;
    ctx.machine_state.stack.push(result);

    Ok(None)
}

/// 0x0b
pub fn signextend(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 2)?;

    let size_in_bytes_minus_one: u8 = stack_items[0]
        .try_into()
        .map_err(|_| EVMError::U256ToU8Error(stack_items[0], ctx.clone()))?;
    let int_to_extend = stack_items[1];

    if size_in_bytes_minus_one >= 32 {
        // cannot extend more
        ctx.machine_state.stack.push(int_to_extend);
        return Ok(None);
    }

    let bit_index = (8 * size_in_bytes_minus_one + 7) as usize;
    // find whether the bit at bit_index is 1 or 0
    let bit = int_to_extend.bit(bit_index);
    // create a mask of 0s up to bit_index and then 1s from then on
    let mask = (U256::one() << bit_index) - U256::one();
    if bit {
        // append 1s to int_to_extend
        ctx.machine_state.stack.push(int_to_extend | !mask);
    } else {
        // append 0s to int_to_extend
        ctx.machine_state.stack.push(int_to_extend & mask);
    };

    Ok(None)
}
