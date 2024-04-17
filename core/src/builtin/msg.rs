use crate::{define_zero, ffi, Address, FromNativeType, U256};

define_zero!(gas, U256, __yul_gas);
define_zero!(address, U256, __yul_gas);
define_zero!(selfbalance, U256, __yul_selfbalance);

define_zero!(caller, Address, __yul_caller);
define_zero!(callvalue, U256, __yul_callvalue);

#[inline]
pub fn calldataload<T>(p: U256) -> T
where
    T: FromNativeType,
{
    let r = unsafe { ffi::__yul_calldataload(p.0) };
    T::from_native_type(r)
}

#[inline]
pub fn calldatasize() -> usize {
    unsafe { ffi::__yul_calldatasize() }
}

#[inline]
pub fn calldatacopy(t: &mut [u8], f: usize) {
    unsafe { ffi::__yul_calldatacopy(t.as_mut_ptr(), f, t.len()) }
}

#[inline]
pub fn codesize() -> usize {
    unsafe { ffi::__yul_codesize() }
}

#[inline]
pub fn codecopy(t: &mut [u8], f: usize) {
    unsafe { ffi::__yul_codecopy(t.as_mut_ptr(), f, t.len()) }
}
