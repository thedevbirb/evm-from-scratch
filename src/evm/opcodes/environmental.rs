use crate::evm::utils::types::{ExecutionContext, OpcodeResult};

/// 0x30
pub fn address(ctx: &mut ExecutionContext) -> OpcodeResult {
    ctx.machine_state.stack.push(ctx.input.address);
    Ok(())
}

/// 0x32
pub fn origin(ctx: &mut ExecutionContext) -> OpcodeResult {
    ctx.machine_state.stack.push(ctx.input.origin);

    Ok(())
}

/// 0x33 Solidity calls this msg.sender
pub fn caller(ctx: &mut ExecutionContext) -> OpcodeResult {
    ctx.machine_state.stack.push(ctx.input.sender);

    Ok(())
}

/// 0x3a
pub fn gasprice(ctx: &mut ExecutionContext) -> OpcodeResult {
    ctx.machine_state.stack.push(ctx.input.price);
    Ok(())
}
