use crate::{
    define_zero,
    ffi::{self, NativeType},
    U256,
};

define_zero!(pc, U256, __yul_pc);

#[inline]
pub fn pop(x: NativeType) {
    unsafe { ffi::__yul_pop(x) }
}
