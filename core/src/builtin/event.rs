use crate::{ffi, AsNativeType};

#[inline]
pub fn log0(d: &[u8]) {
    unsafe { ffi::__yul_log0(d.as_ptr(), d.len()) }
}

#[inline]
pub fn log1(t1: impl AsNativeType, d: &[u8]) {
    unsafe { ffi::__yul_log1(d.as_ptr(), d.len(), t1.as_native_type()) }
}

#[inline]
pub fn log2(t1: impl AsNativeType, t2: impl AsNativeType, d: &[u8]) {
    unsafe {
        ffi::__yul_log2(
            d.as_ptr(),
            d.len(),
            t1.as_native_type(),
            t2.as_native_type(),
        )
    }
}

#[inline]
pub fn log3(t1: impl AsNativeType, t2: impl AsNativeType, t3: impl AsNativeType, d: &[u8]) {
    unsafe {
        ffi::__yul_log3(
            d.as_ptr(),
            d.len(),
            t1.as_native_type(),
            t2.as_native_type(),
            t3.as_native_type(),
        )
    }
}

#[inline]
pub fn log4(
    t1: impl AsNativeType,
    t2: impl AsNativeType,
    t3: impl AsNativeType,
    t4: impl AsNativeType,
    d: &[u8],
) {
    unsafe {
        ffi::__yul_log4(
            d.as_ptr(),
            d.len(),
            t1.as_native_type(),
            t2.as_native_type(),
            t3.as_native_type(),
            t4.as_native_type(),
        )
    }
}
