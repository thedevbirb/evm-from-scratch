use std::collections::HashMap;

use evm_from_scratch_new::evm::{
    evm::EVM,
    utils::{
        errors::EVMError,
        test_types::EvmTest,
        types::{AccountState, BlockHeader, ExecutionContext, Input},
    },
};

use primitive_types::U256;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

fn main() -> Result<(), EVMError> {
    let text = std::fs::read_to_string("./difficult_test.json").unwrap();
    let data: Vec<EvmTest> = serde_json::from_str(&text).unwrap();

    let total = data.len();

    for (index, test) in data.iter().enumerate() {
        println!("Test {} of {}: {}", index + 1, total, test.name);

        let code: Vec<u8> = hex::decode(&test.code.bin).unwrap();

        let mut ctx = ExecutionContext::new();
        if let Some(gs) = &test.state {
            ctx.global_state = gs
                .iter()
                .map(|(k, v)| (U256::from_str_radix(k, 16).unwrap(), AccountState::from(v)))
                .collect()
        };

        if let Some(tx) = &test.tx {
            ctx.input = Input::from(tx);
        }

        if let Some(block) = &test.block {
            ctx.input.block_header = BlockHeader::from(block)
        }

        ctx.input.bytecode = code;

        let result = EVM::execute(&mut ctx)?;

        // Reverse the order of the stack for checking the tests
        ctx.machine_state.stack.reverse();

        let mut expected_stack: Vec<U256> = Vec::new();
        if let Some(ref stacks) = test.expect.stack {
            for value in stacks {
                expected_stack.push(U256::from_str_radix(value, 16).unwrap());
            }
        }

        let mut matching = ctx.machine_state.stack.len() == expected_stack.len();
        if matching {
            for i in 0..ctx.machine_state.stack.len() {
                if ctx.machine_state.stack[i] != expected_stack[i] {
                    matching = false;
                    break;
                }
            }
        }

        matching = matching && result.success == test.expect.success;

        if !matching {
            println!("Instructions: \n{}\n", test.code.asm);

            println!("Expected success: {:?}", test.expect.success);
            println!("Expected stack: [");
            for v in expected_stack {
                println!("  {:#X},", v);
            }
            println!("]\n");

            println!("Actual success: {:?}", result.success);
            println!("Actual stack: [");
            for v in &ctx.machine_state.stack {
                println!("  {:#X},", v);
            }
            println!("]\n");

            println!("\nHint: {}\n", test.hint);
            println!("Progress: {}/{}\n\n", index, total);
            println!("Execution context: {:x?}", ctx);
            panic!("Test failed");
        }
        println!("PASS");
    }
    println!("Congratulations!");
    Ok(())
}
