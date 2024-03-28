use crate::U256;

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

    fn __yul_codesize() -> Cnt;
    fn __yul_codecopy(t: *mut u8, f: Cnt, s: usize);

    fn __yul_returndatasize() -> Cnt;
    fn __yul_returndatacopy(t: *const u8, f: Cnt, s: usize);

}

/// gas still available to execution
pub fn gas() -> U256 {
    U256(unsafe { __yul_gas() })
}

/// contract address at execution
pub fn address() -> U256 {
    U256(unsafe { __yul_address() })
}

pub fn selfbalance() -> U256 {
    U256(unsafe { __yul_selfbalance() })
}

/// caller
pub fn caller() -> U256 {
    U256(unsafe { __yul_caller() })
}

/// value sent to this contract
pub fn callvalue() -> U256 {
    U256(unsafe { __yul_callvalue() })
}

pub fn calldataload(p: U256) -> U256 {
    U256(unsafe { __yul_calldataload(p.0) })
}

pub fn calldatasize() -> U256 {
    U256(unsafe { __yul_calldatasize() })
}

pub fn calldatacopy(data: &mut [u8], f: U256) {
    unsafe { __yul_calldatacopy(data.as_ptr(), f.0, data.len()) }
}

pub fn codesize() -> U256 {
    U256(unsafe { __yul_codesize() })
}

pub fn codecopy(data: &mut [u8], f: U256) {
    unsafe { __yul_codecopy(data.as_mut_ptr(), f.0, data.len()) }
}

pub fn returndatasize() -> U256 {
    U256(unsafe { __yul_returndatasize() })
}

pub fn returndatacopy(data: &mut [u8], f: U256) {
    unsafe { __yul_returndatacopy(data.as_ptr(), f.0, data.len()) }
}
