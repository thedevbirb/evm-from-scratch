use primitive_types::U256;

pub const STOP: u8 = 0x00;
pub const PUSH_0_HEX: u8 = 0x5f;
pub const PUSH_1_HEX: u8 = 0x60;

pub const CHAIN_ID: u8 = 0x01;

pub const BYTES_IN_U256_FROM_ZERO: usize = 0x1f;

/// c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470
pub const KECCAK_EMPTY: U256 = U256([
    0x7bfad8045d85a470,
    0xe500b653ca82273b,
    0x927e7db2dcc703c0,
    0xc5d2460186f7233c,
]);
