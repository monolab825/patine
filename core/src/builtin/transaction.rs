use crate::{Address, U256};

use super::Cnt;

extern "C" {
    fn __yul_call(
        g: Cnt,
        a: Cnt,
        v: Cnt,
        indata: *const u8,
        insize: usize,
        out: *mut u8,
        outsize: usize,
    ) -> Cnt;

    fn __yul_callcode(
        g: Cnt,
        a: Cnt,
        v: Cnt,
        indata: *const u8,
        insize: usize,
        out: *mut u8,
        outsize: usize,
    ) -> Cnt;

    fn __yul_delegatecall(
        g: Cnt,
        a: Cnt,
        indata: *const u8,
        insize: usize,
        out: *mut u8,
        outsize: usize,
    ) -> Cnt;

    fn __yul_staticcall(
        g: Cnt,
        a: Cnt,
        indata: *const u8,
        insize: usize,
        out: *mut u8,
        outsize: usize,
    ) -> Cnt;

    fn __yul_return(p: *const u8, s: usize);
    fn __yul_revert(p: *const u8, s: usize);
    fn __yul_selfdestruct(a: u32);
    fn __yul_invalid();
}

pub fn call(addr: Address, value: U256, gas: U256, input: &[u8], output: &mut [u8]) -> U256 {
    U256(unsafe {
        __yul_call(
            gas.0,
            addr.0,
            value.0,
            input.as_ptr(),
            input.len(),
            output.as_mut_ptr(),
            output.len(),
        )
    })
}

pub fn callcode(addr: Address, value: U256, gas: U256, input: &[u8], output: &mut [u8]) -> U256 {
    U256(unsafe {
        __yul_callcode(
            gas.0,
            addr.0,
            value.0,
            input.as_ptr(),
            input.len(),
            output.as_mut_ptr(),
            output.len(),
        )
    })
}

pub fn delegatecall(addr: Address, gas: U256, input: &[u8], output: &mut [u8]) -> U256 {
    U256(unsafe {
        __yul_delegatecall(
            gas.0,
            addr.0,
            input.as_ptr(),
            input.len(),
            output.as_mut_ptr(),
            output.len(),
        )
    })
}

pub fn staticcall(addr: Address, gas: U256, input: &[u8], output: &mut [u8]) -> U256 {
    U256(unsafe {
        __yul_staticcall(
            gas.0,
            addr.0,
            input.as_ptr(),
            input.len(),
            output.as_mut_ptr(),
            output.len(),
        )
    })
}

pub fn ret(data: &[u8]) {
    unsafe { __yul_return(data.as_ptr(), data.len()) }
}

pub fn revert(data: &[u8]) {
    unsafe { __yul_revert(data.as_ptr(), data.len()) }
}

pub fn selfdestruct(addr: Address) {
    unsafe { __yul_selfdestruct(addr.0) }
}

pub fn invalid() {
    unsafe { __yul_invalid() }
}
