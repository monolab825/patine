use crate::{ffi, Address, Bytes32, U256};

#[inline]
pub fn create(v: U256, code: &[u8]) -> Address {
    Address(unsafe { ffi::__yul_create(v.0, code.as_ptr(), code.len()) })
}

#[inline]
pub fn create2(v: U256, code: &[u8], salt: Bytes32) -> Address {
    Address(unsafe { ffi::__yul_create2(v.0, code.as_ptr(), code.len(), salt.0) })
}

#[inline]
pub fn call(a: Address, g: U256, v: U256, input: &[u8], output: &mut [u8]) -> U256 {
    U256(unsafe {
        ffi::__yul_call(
            g.0,
            a.0,
            v.0,
            input.as_ptr(),
            input.len(),
            output.as_mut_ptr(),
            output.len(),
        )
    })
}

#[inline]
pub fn callcode(a: Address, g: U256, v: U256, input: &[u8], output: &mut [u8]) -> U256 {
    U256(unsafe {
        ffi::__yul_callcode(
            g.0,
            a.0,
            v.0,
            input.as_ptr(),
            input.len(),
            output.as_mut_ptr(),
            output.len(),
        )
    })
}

#[inline]
pub fn delegatecall(a: Address, g: U256, input: &[u8], output: &mut [u8]) -> U256 {
    U256(unsafe {
        ffi::__yul_delegatecall(
            g.0,
            a.0,
            input.as_ptr(),
            input.len(),
            output.as_mut_ptr(),
            output.len(),
        )
    })
}

#[inline]
pub fn staticcall(a: Address, g: U256, input: &[u8], output: &mut [u8]) -> U256 {
    U256(unsafe {
        ffi::__yul_staticcall(
            g.0,
            a.0,
            input.as_ptr(),
            input.len(),
            output.as_mut_ptr(),
            output.len(),
        )
    })
}

#[inline]
pub fn return_call(d: &[u8]) {
    unsafe { ffi::__yul_return(d.as_ptr(), d.len()) }
}

#[inline]
pub fn revert(d: &[u8]) {
    unsafe { ffi::__yul_revert(d.as_ptr(), d.len()) }
}
#[inline]
pub fn selfdestruct(a: Address) {
    unsafe { ffi::__yul_selfdestruct(a.0) }
}

#[inline]
pub fn invalid() {
    unsafe { ffi::__yul_invalid() }
}
