use std::collections::{HashMap, HashSet};

use primitive_types::U256;

use super::{
    constants::{KECCAK_EMPTY, TEST_CONTRACT_ADDRESS, TEST_EOA_ADDRESS},
    errors::EVMError,
};

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
            global_state: get_demo_global_state(),
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

pub fn get_demo_global_state() -> GlobalState {
    let mut gs = GlobalState::new();
    gs.insert(TEST_CONTRACT_ADDRESS, AccountState::new());

    gs
}

pub type Storage = HashMap<U256, U256>;

#[derive(Debug, Clone)]
pub struct MachineState {
    pub pc: usize,
    pub gas: U256,
    pub memory: Vec<u8>,
    pub active_words_memory: usize,
    pub stack: Vec<U256>,

    /// Output data from the previous call from the current environment.
    /// On the yellow paper is formally denoted as $\mu_o$.
    pub output: Vec<u8>,
}

impl MachineState {
    pub fn new() -> MachineState {
        MachineState {
            pc: 0,
            gas: U256::MAX,
            memory: vec![0; 256],
            active_words_memory: 0,
            stack: Vec::new(),
            output: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AccruedSubstate {
    /// a set of accounts that will be discarded following
    /// the transaction's completion
    pub self_destruct_set: HashSet<U256>,

    /// this is a series of archived and indexable 'checkpoints'
    /// in VM code execution that allow for contract calls
    /// to be easily tracked by onlookers external to the
    /// Ethereum world (such as decentralised application front-ends)
    pub logs: Vec<Log>,

    /// the set of touched (modified) accounts, of which the empty ones
    /// are deleted at the end of a transaction
    pub touched_accounts: HashSet<U256>,

    /// the refund balance, increased through using the `SSTORE` instruction
    /// in order to reset contract storage to zero from some non-zero value.
    /// Though not immediately refunded, it is allowed to partially
    /// offset the total execution costs
    pub refund_balance: U256,

    /// the set of accessed account addresses
    pub accessed_accounts: HashSet<U256>,

    /// the set of accessed storage keys
    /// (more accurately, each element of it is a tuple of a
    /// 20-byte account address and a 32-byte storage slot)
    pub accessed_storage_keys: HashSet<(U256, U256)>,
}

impl AccruedSubstate {
    pub fn new() -> AccruedSubstate {
        AccruedSubstate {
            self_destruct_set: HashSet::new(),
            logs: Vec::new(),
            touched_accounts: HashSet::new(),
            refund_balance: U256::zero(),
            accessed_accounts: HashSet::new(),
            accessed_storage_keys: HashSet::new(),
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

    /// the permission to make modifications to the state.
    /// This is also referred as static execution context,
    /// however `static` is a reserved keyword.
    pub write: bool,
}

impl Input {
    /// Returns an Input instance with hardcoded demo data
    pub fn new_demo() -> Input {
        Input {
            address: TEST_CONTRACT_ADDRESS,
            origin: TEST_EOA_ADDRESS,
            price: U256::zero(),
            data: Vec::new(),
            sender: TEST_EOA_ADDRESS,
            value: U256::zero(),
            bytecode: Vec::new(),
            block_header: BlockHeader::new(),
            depth: 0,
            write: true,
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

#[derive(Debug)]
pub struct EVMReturnData {
    pub success: bool,
    pub output: Option<Vec<u8>>,
}

pub type OpcodeResult<'a> = Result<Option<Vec<u8>>, EVMError>;

#[derive(Debug, Clone)]
pub struct Log {
    pub address: U256,
    pub data: Vec<u8>,
    pub topics: Vec<U256>,
}

pub type Logs = Vec<Log>;

pub type Opcode = Box<dyn Fn(&mut ExecutionContext) -> OpcodeResult>;
pub type Opcodes = HashMap<u8, Opcode>;
