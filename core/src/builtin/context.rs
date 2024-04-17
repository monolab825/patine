use crate::{Address, U256};

use super::Cnt;

extern "C" {
    fn __yul_gas() -> Cnt;
    fn __yul_address() -> Cnt;
    fn __yul_selfbalance() -> Cnt;
    fn __yul_caller() -> Cnt;
    fn __yul_callvalue() -> Cnt;

    fn __yul_calldataload(p: Cnt) -> Cnt;
    fn __yul_calldatasize() -> Cnt;
    fn __yul_calldatacopy(t: *const u8, f: Cnt, s: usize);

    fn __yul_codesize() -> usize;
    fn __yul_codecopy(t: *mut u8, f: Cnt, s: usize);

    fn __yul_returndatasize() -> Cnt;
    fn __yul_returndatacopy(t: *const u8, f: Cnt, s: usize);

}

/// gas still available to execution
#[inline]
pub fn gas() -> U256 {
    U256(unsafe { __yul_gas() })
}

/// contract address at execution
#[inline]
pub fn address() -> Address {
    Address(unsafe { __yul_address() })
}

#[inline]
pub fn selfbalance() -> U256 {
    U256(unsafe { __yul_selfbalance() })
}

/// caller
#[inline]
pub fn caller() -> Address {
    Address(unsafe { __yul_caller() })
}

/// value sent to this contract
#[inline]
pub fn callvalue() -> U256 {
    U256(unsafe { __yul_callvalue() })
}

#[inline]
pub fn calldataload(p: U256) -> U256 {
    U256(unsafe { __yul_calldataload(p.0) })
}

#[inline]
pub fn calldatasize() -> U256 {
    U256(unsafe { __yul_calldatasize() })
}

#[inline]
pub fn calldatacopy(data: &mut [u8], f: U256) {
    unsafe { __yul_calldatacopy(data.as_ptr(), f.0, data.len()) }
}

#[inline]
pub fn codesize() -> usize {
    unsafe { __yul_codesize() }
}

#[inline]
pub fn codecopy(data: &mut [u8], f: U256) {
    unsafe { __yul_codecopy(data.as_mut_ptr(), f.0, data.len()) }
}

#[inline]
pub fn returndatasize() -> U256 {
    U256(unsafe { __yul_returndatasize() })
}

#[inline]
pub fn returndatacopy(data: &mut [u8], f: U256) {
    unsafe { __yul_returndatacopy(data.as_ptr(), f.0, data.len()) }
}
