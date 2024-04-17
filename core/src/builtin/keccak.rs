use crate::{ffi::__yul_keccak256, U256};

#[inline]
pub fn keccak256(data: &[u8]) -> U256 {
    U256(unsafe { __yul_keccak256(data.as_ptr(), data.len()) })
}
