use crate::{define_two_op, ffi, U256};

// operations

define_two_op!(add, U256, U256, U256, __yul_add);
define_two_op!(sub, U256, U256, U256, __yul_sub);
define_two_op!(mul, U256, U256, U256, __yul_mul);
define_two_op!(udiv, U256, U256, U256, __yul_div);
define_two_op!(sdiv, U256, U256, U256, __yul_sdiv);
define_two_op!(umod, U256, U256, U256, __yul_mod);
define_two_op!(smod, U256, U256, U256, __yul_smod);
define_two_op!(exp, U256, U256, U256, __yul_exp);
#[inline]
pub fn addmod(x: U256, y: U256, m: U256) -> U256 {
    U256(unsafe { ffi::__yul_addmod(x.0, y.0, m.0) })
}
#[inline]
pub fn mulmod(x: U256, y: U256, m: U256) -> U256 {
    U256(unsafe { ffi::__yul_mulmod(x.0, y.0, m.0) })
}

// compare

define_two_op!(lt, U256, U256, U256, __yul_lt);
define_two_op!(gt, U256, U256, U256, __yul_gt);
define_two_op!(slt, U256, U256, U256, __yul_slt);
define_two_op!(sgt, U256, U256, U256, __yul_sgt);
define_two_op!(eq, U256, U256, U256, __yul_eq);
#[inline]
pub fn iszero(x: U256) -> bool {
    unsafe { ffi::__yul_iszero(x.0) }
}

// bit operations

#[inline]
pub fn not(x: U256) -> U256 {
    U256(unsafe { ffi::__yul_not(x.0) })
}
define_two_op!(and, U256, U256, U256, __yul_and);
define_two_op!(or, U256, U256, U256, __yul_or);
define_two_op!(xor, U256, U256, U256, __yul_xor);
define_two_op!(byte, U256, U256, U256, __yul_byte);

// bit shifts

define_two_op!(shl, U256, U256, U256, __yul_shl);
define_two_op!(shr, U256, U256, U256, __yul_shr);
define_two_op!(sar, U256, U256, U256, __yul_sar);

define_two_op!(signextend, U256, U256, U256, __yul_signextend);
