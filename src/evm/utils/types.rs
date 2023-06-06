use std::collections::{HashMap, HashSet};

use primitive_types::U256;

pub type Address = String;

pub struct AccountState {
    pub nonce: usize,
    pub balance: U256,
    //todo should add codeHash?
}

impl AccountState {
    pub fn new() -> AccountState {
        AccountState {
            nonce: 0,
            balance: U256::zero(),
        }
    }
}

pub type GlobalState = HashMap<Address, AccountState>;

pub struct MachineState {
    pub pc: usize,
    pub gas: U256,
    pub memory: Vec<U256>,
    pub active_words_memory_idx: usize,
    pub storage: HashMap<U256, U256>,
    pub stack: Vec<U256>,
}

impl MachineState {
    pub fn new() -> MachineState {
        MachineState {
            pc: 0,
            gas: U256::MAX,
            memory: vec![U256::from(u16::MAX), U256::zero()],
            active_words_memory_idx: 0,
            storage: HashMap::new(),
            stack: Vec::new(),
        }
    }
}

pub struct AccruedSubstate {
    self_destruct_set: HashSet<Address>,
    logs: Vec<Log>,
    touched_accounts: HashSet<Address>,
    refund_balance: U256,
    accessed_account: HashSet<Address>,
    accessed_storage_keys: HashMap<Address, U256>,
}

impl AccruedSubstate {
    pub fn new() -> AccruedSubstate {
        AccruedSubstate {
            self_destruct_set: HashSet::new(),
            logs: Vec::new(),
            touched_accounts: HashSet::new(),
            refund_balance: U256::zero(),
            accessed_account: HashSet::new(),
            accessed_storage_keys: HashMap::new(),
        }
    }
}

pub struct Input {
    ///  the address of the account which owns the code that is executing
    pub code_owner: Address,

    /// the sender address of the transaction that originated this execution
    pub origin: Address,

    /// the price of gas in the transaction that originated this execution
    pub price: U256,

    /// the byte array that is the input data to this execution;
    /// if the execution agent is a transaction, this would
    /// be the transaction data
    pub data: Vec<u8>,

    /// the address of the account which caused the
    /// code to be executing; if the execution agent is a
    /// transaction, this would be the transaction sender
    pub sender: Address,

    /// the value, in Wei, passed to this account as
    /// part of the same procedure as execution; if the
    /// execution agent is a transaction, this would be the
    /// transaction value
    pub value: U256,

    /// the value, in Wei, passed to this account as
    /// part of the same procedure as execution; if the
    /// execution agent is a transaction, this would be the
    /// transaction value
    pub bytecode: Vec<u8>,

    /// the depth of the present message-call or
    /// contract-creation (i.e. the number of CALLs or
    /// CREATE(2)s being executed at present)
    pub depth: usize,
}

impl Input {
    /// Returns an Input instance with hardcoded demo data
    pub fn new_demo() -> Input {
        Input {
            code_owner: String::from("0x8731d54E9D02c286767d56ac03e8037C07e01e98"),
            origin: String::from("0xc2e9A90a9B957c4687c5944491f86E29C10Cb439"),
            price: U256::zero(),
            data: Vec::new(),
            sender: String::from("0xc2e9A90a9B957c4687c5944491f86E29C10Cb439"),
            value: U256::zero(),
            bytecode: Vec::new(),
            depth: 0,
        }
    }
}

pub struct Log {
    address: Address,
    data: String,
    topics: Vec<String>,
}

pub type Opcodes = HashMap<u8, ()>;
