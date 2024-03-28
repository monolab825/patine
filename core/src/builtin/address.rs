use crate::{Address, U256};

use super::Cnt;

extern "C" {
    fn __yul_balance(a: Cnt) -> Cnt;

    fn __yul_extcodesize(a: Cnt) -> Cnt;
    fn __yul_extcodecopy(a: Cnt, t: *mut u8, f: Cnt, s: usize);
    fn __yul_extcodehash(a: Cnt) -> Cnt;

}

#[inline]
pub fn balance(addr: Address) -> U256 {
    U256(unsafe { __yul_balance(addr.0) })
}

#[inline]
pub fn extcodesize(addr: Address) -> U256 {
    U256(unsafe { __yul_extcodesize(addr.0) })
}

#[inline]
pub fn extcodecopy(addr: Address, data: &mut [u8], from: U256) {
    unsafe { __yul_extcodecopy(addr.0, data.as_mut_ptr(), from.0, data.len()) }
}

#[inline]
pub fn extcodehash(addr: Address) -> U256 {
    U256(unsafe { __yul_extcodehash(addr.0) })
}
