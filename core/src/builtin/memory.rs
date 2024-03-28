use crate::U256;

use super::Cnt;

extern "C" {
    fn __yul_mload(p: *const u8) -> Cnt;
    fn __yul_mstore(p: *mut u8, v: Cnt);
    fn __yul_mstore8(p: *mut u8, v: Cnt);
}

#[inline]
pub fn mload(a: &[u8]) -> U256 {
    U256(unsafe { __yul_mload(a.as_ptr()) })
}

#[inline]
pub fn mstore(v: U256, target: &mut [u8]) {
    unsafe { __yul_mstore(target.as_mut_ptr(), v.0) }
}

#[inline]
pub fn mstore8(v: U256, target: &mut [u8]) {
    unsafe { __yul_mstore(target.as_mut_ptr(), v.0) }
}
