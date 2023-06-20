use primitive_types::U256;

use crate::evm::utils::{
    errors::EVMError,
    types::{ExecutionContext, OpcodeResult},
};

/// 0x30
pub fn address(ctx: &mut ExecutionContext) -> OpcodeResult {
    let address = &ctx.input.address;
    let address = U256::from_str_radix(address, 16)
        .map_err(|_| EVMError::FromStrRadixError(address.clone(), ctx.clone()))?;

    ctx.machine_state.stack.push(address);

    Ok(())
}

/// 0x32
pub fn origin(ctx: &mut ExecutionContext) -> OpcodeResult {
    let origin = &ctx.input.origin;
    let origin = U256::from_str_radix(origin, 16)
        .map_err(|_| EVMError::FromStrRadixError(origin.clone(), ctx.clone()))?;

    ctx.machine_state.stack.push(origin);

    Ok(())
}

/// 0x33 Solidity calls this msg.sender
pub fn caller(ctx: &mut ExecutionContext) -> OpcodeResult {
    let sender = &ctx.input.sender;
    let sender = U256::from_str_radix(sender, 16)
        .map_err(|_| EVMError::FromStrRadixError(sender.clone(), ctx.clone()))?;

    ctx.machine_state.stack.push(sender);

    Ok(())
}

/// 0x3a
pub fn gasprice(ctx: &mut ExecutionContext) -> OpcodeResult {
    let price = ctx.input.price.clone();
    ctx.machine_state.stack.push(price);
    Ok(())
}
