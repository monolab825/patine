use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

use crate::{ffi::NativeType, *};

macro_rules! define_uint {
    ($ty:ident, $($ft:ty),*) => {
        #[repr(C)]
        #[repr(align(32))]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $ty(pub(crate) NativeType);

        impl AsNativeType for $ty {
            fn as_native_type(&self) -> NativeType {
                self.0
            }
        }

        impl FromNativeType for $ty {
            fn from_native_type(x: NativeType) -> Self {
                Self(x)
            }
        }

        impl Integer for $ty {}

        $(
            impl From<$ft> for $ty {
                #[inline]
                fn from(value: $ft) -> Self {
                    Self(value.0)
                }
            }
        )*


        define_two_op_trait!($ty, Add, add, add);
        define_two_assign_op_trait!($ty, AddAssign, add, add_assign);

        define_two_op_trait!($ty, Sub, sub, sub);
        define_two_assign_op_trait!($ty, SubAssign, sub, sub_assign);

        define_two_op_trait!($ty, Mul, mul, mul);
        define_two_assign_op_trait!($ty, MulAssign, mul, mul_assign);

        define_two_op_trait!($ty, Div, udiv, div);
        define_two_assign_op_trait!($ty, DivAssign, udiv, div_assign);

        define_two_op_trait!($ty, Rem, umod, rem);
        define_two_assign_op_trait!($ty, RemAssign, umod, rem_assign);

        define_two_op_trait!($ty, Shr, shr, shr);
        define_two_assign_op_trait!($ty, ShrAssign, shr, shr_assign);

        define_two_op_trait!($ty, Shl, shl, shl);
        define_two_assign_op_trait!($ty, ShlAssign, shl, shl_assign);

        define_two_op_trait!($ty, BitAnd, bitand, bitand);
        define_two_assign_op_trait!($ty, BitAndAssign, bitand, bitand_assign);

        define_two_op_trait!($ty, BitOr, bitor, bitor);
        define_two_assign_op_trait!($ty, BitOrAssign, bitor, bitor_assign);

        define_two_op_trait!($ty, BitXor, bitxor, bitxor);
        define_two_assign_op_trait!($ty, BitXorAssign, bitxor, bitxor_assign);

        impl Not for $ty {
            type Output = Self;

            #[inline]
            fn not(self) -> Self::Output {
                builtin::not(self)
            }
        }
    };
}

define_uint!(U8, Bytes1, S8);
define_uint!(U16, Bytes2, S16);
define_uint!(U24, Bytes3, S24);
define_uint!(U32, Bytes4, S32);
define_uint!(U40, Bytes5, S40);
define_uint!(U48, Bytes6, S48);
define_uint!(U56, Bytes7, S56);
define_uint!(U64, Bytes8, S64);
define_uint!(U72, Bytes9, S72);
define_uint!(U80, Bytes10, S80);
define_uint!(U88, Bytes11, S88);
define_uint!(U96, Bytes12, S96);
define_uint!(U104, Bytes13, S104);
define_uint!(U112, Bytes14, S112);
define_uint!(U120, Bytes15, S120);
define_uint!(U128, Bytes16, S128);
define_uint!(U136, Bytes17, S136);
define_uint!(U144, Bytes18, S144);
define_uint!(U152, Bytes19, S152);
define_uint!(U160, Bytes20, S160);
define_uint!(U168, Bytes21, S168);
define_uint!(U176, Bytes22, S176);
define_uint!(U184, Bytes23, S184);
define_uint!(U192, Bytes24, S192);
define_uint!(U200, Bytes25, S200);
define_uint!(U208, Bytes26, S208);
define_uint!(U216, Bytes27, S216);
define_uint!(U224, Bytes28, S224);
define_uint!(U232, Bytes29, S232);
define_uint!(U240, Bytes30, S240);
define_uint!(U248, Bytes31, S248);
define_uint!(U256, Bytes32, S256);
