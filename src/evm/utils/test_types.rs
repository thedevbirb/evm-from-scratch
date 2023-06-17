use std::{collections::HashMap, usize};

use primitive_types::U256;
use serde::Deserialize;

use super::{
    constants::KECCAK_EMPTY,
    helpers::bytes_from_hex_str,
    types::{AccountState, Address},
};

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
