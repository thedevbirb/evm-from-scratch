use std::collections::{HashMap, HashSet};

use primitive_types::U256;

use super::{constants::KECCAK_EMPTY, errors::EVMError};

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub global_state: GlobalState,
    pub machine_state: MachineState,
    pub accrued_substate: AccruedSubstate,
    pub input: Input,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            global_state: GlobalState::new(),
            machine_state: MachineState::new(),
            accrued_substate: AccruedSubstate::new(),
            input: Input::new_demo(),
        }
    }
}

pub type Address = String;

#[derive(Debug, Clone)]
pub struct AccountState {
    pub nonce: usize,
    pub balance: U256,
    pub code_hash: U256,
    pub code: Vec<u8>,
    pub storage_root: U256,
    pub storage: Storage,
}

impl AccountState {
    pub fn new() -> AccountState {
        AccountState {
            nonce: 0,
            balance: U256::zero(),
            code_hash: KECCAK_EMPTY,
            code: Vec::new(),
            storage_root: KECCAK_EMPTY,
            storage: HashMap::new(),
        }
    }
}

pub type GlobalState = HashMap<Address, AccountState>;
pub type Storage = HashMap<U256, U256>;

#[derive(Debug, Clone)]
pub struct MachineState {
    pub pc: usize,
    pub gas: U256,
    pub memory: Vec<u8>,
    pub active_words_memory: usize,
    pub storage: HashMap<U256, U256>,
    pub stack: Vec<U256>,
}

impl MachineState {
    pub fn new() -> MachineState {
        MachineState {
            pc: 0,
            gas: U256::MAX,
            memory: vec![0; 256],
            active_words_memory: 0,
            storage: HashMap::new(),
            stack: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Input {
    ///  the address of the account which owns the code that is executing
    pub address: Address,

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

    /// the byte array that is the machine code to be executed.
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
            address: String::from("0x8731d54E9D02c286767d56ac03e8037C07e01e98"),
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

pub struct Output {
    pub success: bool,
}

pub struct EVMReturnData {
    pub ctx: ExecutionContext,
    pub output: Output,
}

pub type OpcodeResult<'a> = Result<(), EVMError>;

#[derive(Debug, Clone)]
pub struct Log {
    address: Address,
    data: String,
    topics: Vec<String>,
}

pub type Logs = Vec<Log>;

pub type Opcode = Box<dyn Fn(&mut ExecutionContext) -> OpcodeResult>;
pub type Opcodes = HashMap<u8, Opcode>;
