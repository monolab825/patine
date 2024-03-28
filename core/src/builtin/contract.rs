use crate::{Address, Bytes32, U256};

use super::Cnt;

extern "C" {
    fn __yul_create(v: Cnt, p: *const u8, n: usize) -> Cnt;
    fn __yul_create2(v: Cnt, p: *const u8, n: usize, s: Cnt) -> Cnt;

}

pub fn create(value: U256, data: &[u8]) -> Address {
    Address(unsafe { __yul_create(value.0, data.as_ptr(), data.len()) })
}

pub fn create2(value: U256, data: &[u8], salt: Bytes32) -> Address {
    Address(unsafe { __yul_create2(value.0, data.as_ptr(), data.len(), salt.0) })
}
