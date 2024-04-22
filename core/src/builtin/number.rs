use crate::{define_two_op, define_two_op_native_return, ffi, AsNativeType, FromNativeType};

// operations

define_two_op!(add, __yul_add);
define_two_op!(sub, __yul_sub);
define_two_op!(mul, __yul_mul);
define_two_op!(udiv, __yul_div);
define_two_op!(sdiv, __yul_sdiv);
define_two_op!(umod, __yul_mod);
define_two_op!(smod, __yul_smod);
define_two_op!(exp, __yul_exp);
#[inline]
pub fn addmod<R>(x: impl AsNativeType, y: impl AsNativeType, m: impl AsNativeType) -> R
where
    R: FromNativeType,
{
    R::from_native_type(unsafe {
        ffi::__yul_addmod(x.as_native_type(), y.as_native_type(), m.as_native_type())
    })
}

#[inline]
pub fn mulmod<R>(x: impl AsNativeType, y: impl AsNativeType, m: impl AsNativeType) -> R
where
    R: FromNativeType,
{
    R::from_native_type(unsafe {
        ffi::__yul_mulmod(x.as_native_type(), y.as_native_type(), m.as_native_type())
    })
}

// compare

define_two_op_native_return!(lt, __yul_lt, bool);
define_two_op_native_return!(gt, __yul_gt, bool);
define_two_op_native_return!(slt, __yul_slt, bool);
define_two_op_native_return!(sgt, __yul_sgt, bool);
define_two_op_native_return!(eq, __yul_eq, bool);

pub fn iszero(x: impl AsNativeType) -> bool {
    unsafe { ffi::__yul_iszero(x.as_native_type()) }
}

// bit operations

#[inline]
pub fn not<R>(x: impl AsNativeType) -> R
where
    R: FromNativeType,
{
    R::from_native_type(unsafe { ffi::__yul_not(x.as_native_type()) })
}
define_two_op!(bitand, __yul_and);
define_two_op!(bitor, __yul_or);
define_two_op!(bitxor, __yul_xor);
define_two_op!(byte, __yul_byte);

// bit shifts

define_two_op!(shl, __yul_shl);
define_two_op!(shr, __yul_shr);
define_two_op!(sar, __yul_sar);

define_two_op!(signextend, __yul_signextend);
