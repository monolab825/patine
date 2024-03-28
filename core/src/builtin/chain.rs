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

#[inline]
pub fn chainid() -> U256 {
    U256(unsafe { __yul_chainid() })
}

#[inline]
pub fn basefee() -> U256 {
    U256(unsafe { __yul_basefee() })
}

#[inline]
pub fn blobbasefee() -> U256 {
    U256(unsafe { __yul_blobbasefee() })
}

#[inline]
pub fn origin() -> U256 {
    U256(unsafe { __yul_origin() })
}

#[inline]
pub fn blockhash(number: U256) -> Bytes32 {
    Bytes32(unsafe { __yul_blobhash(number.0) })
}

#[inline]
pub fn blobhash(number: U256) -> Bytes32 {
    Bytes32(unsafe { __yul_blobhash(number.0) })
}

#[inline]
pub fn coinbase() -> Address {
    Address(unsafe { __yul_coinbase() })
}

#[inline]
pub fn timestamp() -> U256 {
    U256(unsafe { __yul_timestamp() })
}

#[inline]
pub fn number() -> U256 {
    U256(unsafe { __yul_number() })
}

#[inline]
pub fn diffculty() -> U256 {
    U256(unsafe { __yul_difficulty() })
}

#[inline]
pub fn prevrandao() -> Bytes32 {
    Bytes32(unsafe { __yul_prevrandao() })
}

#[inline]
pub fn gaslimit() -> U256 {
    U256(unsafe { __yul_gaslimit() })
}
