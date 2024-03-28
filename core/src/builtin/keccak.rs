use crate::U256;

use super::Cnt;

extern "C" {
    fn __yul_keccak256(ptr: *const u8, len: usize) -> Cnt;

}

#[inline]
pub fn keccak256(data: &[u8]) -> U256 {
    U256(unsafe { __yul_keccak256(data.as_ptr(), data.len()) })
}
