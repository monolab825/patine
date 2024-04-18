use crate::{Address, Bytes32, U256};

#[inline]
pub fn create(value: U256, data: &[u8]) -> Address {
    Address(unsafe { __yul_create(value.0, data.as_ptr(), data.len()) })
}

#[inline]
pub fn create2(value: U256, data: &[u8], salt: Bytes32) -> Address {
    Address(unsafe { __yul_create2(value.0, data.as_ptr(), data.len(), salt.0) })
}
