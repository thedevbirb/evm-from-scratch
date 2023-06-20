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

pub type GlobalState = HashMap<U256, AccountState>;
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
    self_destruct_set: HashSet<U256>,
    logs: Vec<Log>,
    touched_accounts: HashSet<U256>,
    refund_balance: U256,
    accessed_account: HashSet<U256>,
    accessed_storage_keys: HashMap<U256, U256>,
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

/// Input has defined in the yellow paper
#[derive(Debug, Clone)]
pub struct Input {
    ///  the address of the account which owns the code that is executing.
    ///  Also referred as `to`
    pub address: U256,

    /// the sender address of the transaction that originated this execution
    pub origin: U256,

    /// the price of gas in the transaction that originated this execution
    pub price: U256,

    /// the byte array that is the input data to this execution;
    /// if the execution agent is a transaction, this would
    /// be the transaction data
    pub data: Vec<u8>,

    /// the address of the account which caused the
    /// code to be executing; if the execution agent is a
    /// transaction, this would be the transaction sender
    pub sender: U256,

    /// the value, in Wei, passed to this account as
    /// part of the same procedure as execution; if the
    /// execution agent is a transaction, this would be the
    /// transaction value
    pub value: U256,

    /// the block header of the present block
    pub block_header: BlockHeader,

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
            address: U256::from_str_radix("0x8731d54E9D02c286767d56ac03e8037C07e01e98", 16)
                .unwrap(),
            origin: U256::from_str_radix("0xc2e9A90a9B957c4687c5944491f86E29C10Cb439", 16).unwrap(),
            price: U256::zero(),
            data: Vec::new(),
            sender: U256::from_str_radix("0xc2e9A90a9B957c4687c5944491f86E29C10Cb439", 16).unwrap(),
            value: U256::zero(),
            bytecode: Vec::new(),
            block_header: BlockHeader::new(),
            depth: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockHeader {
    /// The 160-bit address to which all fees collected from the successful
    /// mining of this block be transferred
    pub beneficiary: U256,

    /// A scalar value corresponding to the difficulty level of this block.
    /// This can be calculated from the previous block’s difficulty
    /// level and the timestamp
    pub difficulty: U256,

    /// A scalar value equal to the number of ancestor blocks.
    /// The genesis block has a number of zero
    pub number: U256,

    /// A scalar value equal to the reasonable output of Unix’s time()
    /// at this block’s inception
    pub timestamp: U256,

    /// A scalar value equal to the current limit of gas expenditure per block
    pub gas_limit: U256,

    pub base_fee: U256,
}

impl BlockHeader {
    pub fn new() -> Self {
        Self {
            beneficiary: U256::zero(),
            difficulty: U256::zero(),
            number: U256::zero(),
            timestamp: U256::zero(),
            gas_limit: U256::zero(),
            base_fee: U256::zero(),
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
    address: U256,
    data: String,
    topics: Vec<String>,
}

pub type Logs = Vec<Log>;

pub type Opcode = Box<dyn Fn(&mut ExecutionContext) -> OpcodeResult>;
pub type Opcodes = HashMap<u8, Opcode>;
