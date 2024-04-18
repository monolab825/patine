use crate::{ffi::__yul_keccak256, Bytes32};

#[inline]
pub fn keccak256(data: &[u8]) -> Bytes32 {
    Bytes32(unsafe { __yul_keccak256(data.as_ptr(), data.len()) })
}
