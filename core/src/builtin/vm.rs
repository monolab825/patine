use crate::{define_zero, ffi, U256};

define_zero!(pc, U256, __yul_pc);

#[inline]
pub fn pop(x: U256) {
    unsafe { ffi::__yul_pop(x.0) }
}
