use std::{collections::HashMap, usize};

use primitive_types::U256;
use serde::Deserialize;

use super::{
    constants::KECCAK_EMPTY,
    helpers::bytes_from_hex_str,
    types::{AccountState, BlockHeader, Input},
};

pub type Address = String;
pub type State = HashMap<Address, TestAccountState>;

#[derive(Debug, Deserialize)]
pub struct TestAccountState {
    pub nonce: Option<String>,
    pub balance: Option<String>,
    pub code: Option<StateCode>,
}

impl From<&TestAccountState> for AccountState {
    fn from(account_state: &TestAccountState) -> Self {
        AccountState {
            nonce: match &account_state.nonce {
                Some(n) => usize::from_str_radix(&n, 16).unwrap_or(0),
                None => 0,
            },
            balance: match &account_state.balance {
                Some(b) => U256::from_str_radix(&b, 16).unwrap_or(U256::zero()),
                None => U256::zero(),
            },
            code: match &account_state.code {
                Some(c) => bytes_from_hex_str(&c.bin, false).unwrap_or(Vec::new()),
                None => Vec::new(),
            },
            code_hash: KECCAK_EMPTY,
            storage: HashMap::new(),
            storage_root: KECCAK_EMPTY,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct EvmTest {
    pub name: String,
    pub hint: String,
    pub code: Code,
    pub tx: Option<TxData>,
    pub block: Option<BlockData>,
    pub state: Option<State>,
    pub expect: Expect,
}

#[derive(Debug, Deserialize)]
pub struct StateCode {
    pub asm: Option<String>,
    pub bin: String,
}

#[derive(Debug, Deserialize)]
pub struct Code {
    pub asm: String,
    pub bin: String,
}

#[derive(Debug, Deserialize)]
pub struct Expect {
    pub stack: Option<Vec<String>>,
    pub success: bool,
}

#[derive(Debug, Deserialize)]
pub struct TxData {
    pub from: Option<String>,
    pub to: Option<String>,
    pub origin: Option<String>,
    pub gasprice: Option<String>,
    pub value: Option<String>,
    pub data: Option<String>,
}

impl From<&TxData> for Input {
    fn from(value: &TxData) -> Self {
        let mut input = Input::new_demo();

        if let Some(to) = &value.to {
            input.address = U256::from_str_radix(to, 16).unwrap();
        }
        if let Some(from) = &value.from {
            input.sender = U256::from_str_radix(from, 16).unwrap();
        }
        if let Some(origin) = &value.origin {
            input.origin = U256::from_str_radix(origin, 16).unwrap();
        }
        if let Some(gasprice) = &value.gasprice {
            input.price = U256::from_str_radix(gasprice, 16).unwrap();
        }

        input
    }
}

#[derive(Debug, Deserialize)]
pub struct BlockData {
    pub basefee: Option<String>,
    pub coinbase: Option<String>,
    pub timestamp: Option<String>,
    pub number: Option<String>,
    pub gaslimit: Option<String>,
    pub difficulty: Option<String>,
    pub chainid: Option<String>,
}

impl From<&BlockData> for BlockHeader {
    fn from(value: &BlockData) -> Self {
        let mut block_header = BlockHeader::new();
        if let Some(c) = &value.coinbase {
            block_header.beneficiary = U256::from_str_radix(c, 16).unwrap();
        }
        if let Some(d) = &value.difficulty {
            block_header.difficulty = U256::from_str_radix(d, 16).unwrap();
        }
        if let Some(n) = &value.number {
            block_header.number = U256::from_str_radix(n, 16).unwrap();
        }
        if let Some(t) = &value.timestamp {
            block_header.timestamp = U256::from_str_radix(t, 16).unwrap();
        }
        if let Some(l) = &value.gaslimit {
            block_header.gas_limit = U256::from_str_radix(l, 16).unwrap();
        }
        block_header
    }
}
