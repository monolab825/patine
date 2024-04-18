use crate::{
    ffi::{__yul_mcopy, __yul_mload, __yul_msize, __yul_mstore, __yul_mstore8},
    AsNativeType, FromNativeType,
};

#[inline]
pub fn mload<T>(a: &[u8]) -> T
where
    T: FromNativeType,
{
    let r = unsafe { __yul_mload(a.as_ptr()) };
    T::from_native_type(r)
}

#[inline]
pub fn mstore<T>(v: T, target: &mut [u8])
where
    T: AsNativeType,
{
    unsafe { __yul_mstore(target.as_mut_ptr(), v.as_native_type()) }
}

#[inline]
pub fn mstore8<T>(v: T, target: &mut [u8])
where
    T: AsNativeType,
{
    unsafe { __yul_mstore8(target.as_mut_ptr(), v.as_native_type()) }
}

#[inline]
pub fn msize() -> usize {
    unsafe { __yul_msize() }
}

/// # Safety
#[inline]
pub unsafe fn copy(t: *mut u8, f: *const u8, s: usize) {
    __yul_mcopy(t, f, s)
}
