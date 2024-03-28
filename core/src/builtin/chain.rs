use crate::{Address, Bytes32, U256};

use super::Cnt;

extern "C" {
    fn __yul_chainid() -> Cnt;
    fn __yul_basefee() -> Cnt;
    fn __yul_blobbasefee() -> Cnt;
    fn __yul_origin() -> Cnt;
    fn __yul_blockhash(b: Cnt) -> Cnt;
    fn __yul_blobhash(i: Cnt) -> Cnt;
    fn __yul_coinbase() -> Cnt;
    fn __yul_timestamp() -> Cnt;
    fn __yul_number() -> Cnt;
    fn __yul_difficulty() -> Cnt;
    fn __yul_prevrandao() -> Cnt;
    fn __yul_gaslimit() -> Cnt;
}

pub fn chainid() -> U256 {
    U256(unsafe { __yul_chainid() })
}

pub fn basefee() -> U256 {
    U256(unsafe { __yul_basefee() })
}

pub fn blobbasefee() -> U256 {
    U256(unsafe { __yul_blobbasefee() })
}

pub fn origin() -> U256 {
    U256(unsafe { __yul_origin() })
}

pub fn blockhash(number: U256) -> Bytes32 {
    Bytes32(unsafe { __yul_blobhash(number.0) })
}

pub fn blobhash(number: U256) -> Bytes32 {
    Bytes32(unsafe { __yul_blobhash(number.0) })
}

pub fn coinbase() -> Address {
    Address(unsafe { __yul_coinbase() })
}

pub fn timestamp() -> U256 {
    U256(unsafe { __yul_timestamp() })
}

pub fn number() -> U256 {
    U256(unsafe { __yul_number() })
}

pub fn diffculty() -> U256 {
    U256(unsafe { __yul_difficulty() })
}

pub fn prevrandao() -> Bytes32 {
    Bytes32(unsafe { __yul_prevrandao() })
}

pub fn gaslimit() -> U256 {
    U256(unsafe { __yul_gaslimit() })
}
