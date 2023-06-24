use primitive_types::U256;

pub const STOP: u8 = 0x00;
pub const PUSH_0: u8 = 0x5f;
pub const PUSH_1: u8 = 0x60;
pub const DUP_1: u8 = 0x80;
pub const LOG_0: u8 = 0xa0;
pub const REVERT: u8 = 0xfd;

pub const CHAIN_ID: u8 = 0x01;

pub const CALL_DEPTH_LIMIT: usize = 1024;

pub const BYTES_IN_U256_FROM_ZERO: usize = 0x1f;

/// c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470
pub const KECCAK_EMPTY: U256 = U256([
    0x7bfad8045d85a470,
    0xe500b653ca82273b,
    0x927e7db2dcc703c0,
    0xc5d2460186f7233c,
]);

pub const TEST_CONTRACT_ADDRESS: U256 =
    //    U256([0x0, 0x7C07e01e98, 0x767d56ac03e8037C, 0x8731d54E9D02c286]);
    //    U256([0x0, 0x1000000000, 0x0000000000000000, 0x0000000000000aaa]);
    U256([0x0, 0xdddddddddd, 0xdddddddddddddddd, 0xdddddddddddddddd]);
pub const TEST_EOA_ADDRESS: U256 =
    U256([0x0, 0x29C10Cb439, 0x4687c5944491f86E, 0xc2e9A90a9B957c46]);
