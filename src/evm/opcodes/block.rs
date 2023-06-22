use primitive_types::U256;

use crate::evm::utils::{
    constants::CHAIN_ID,
    helpers::pop_n,
    types::{ExecutionContext, OpcodeResult},
};

// I should implement block header data inside Input. See yellow paper
// block opcodes to see what should be inside

/// 0x40
/// Not implemented for this test suite
pub fn blockhash(ctx: &mut ExecutionContext) -> OpcodeResult {
    let _block_number = pop_n(ctx, 1)?[0];
    ctx.machine_state.stack.push(U256::zero());
    Ok(())
}

/// 0x41
pub fn coinbase(ctx: &mut ExecutionContext) -> OpcodeResult {
    let beneficiary = ctx.input.block_header.beneficiary;
    ctx.machine_state.stack.push(beneficiary);
    Ok(())
}

/// 0x42
pub fn timestamp(ctx: &mut ExecutionContext) -> OpcodeResult {
    let timestamp = ctx.input.block_header.timestamp;
    ctx.machine_state.stack.push(timestamp);
    Ok(())
}

/// 0x43
pub fn number(ctx: &mut ExecutionContext) -> OpcodeResult {
    let number = ctx.input.block_header.number;
    ctx.machine_state.stack.push(number);
    Ok(())
}

/// 0x44
pub fn difficulty(ctx: &mut ExecutionContext) -> OpcodeResult {
    let difficulty = ctx.input.block_header.difficulty;
    ctx.machine_state.stack.push(difficulty);
    Ok(())
}

/// 0x45
pub fn gaslimit(ctx: &mut ExecutionContext) -> OpcodeResult {
    let gas_limit = ctx.input.block_header.gas_limit;
    ctx.machine_state.stack.push(gas_limit);
    Ok(())
}

/// 0x46
pub fn chain(ctx: &mut ExecutionContext) -> OpcodeResult {
    let chain_id = U256::from(CHAIN_ID);
    ctx.machine_state.stack.push(chain_id);
    Ok(())
}

/// 0x47
pub fn selfbalance(ctx: &mut ExecutionContext) -> OpcodeResult {
    let to = ctx.input.address;
    let balance = if let Some(account_state) = ctx.global_state.get(&to) {
        account_state.balance.clone()
    } else {
        U256::zero()
    };

    ctx.machine_state.stack.push(balance);

    Ok(())
}

/// 0x48
pub fn basefee(ctx: &mut ExecutionContext) -> OpcodeResult {
    let base_fee = ctx.input.block_header.base_fee;
    ctx.machine_state.stack.push(base_fee);
    Ok(())
}
