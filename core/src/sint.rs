use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

use crate::{ffi::NativeType, *};

macro_rules! define_sint {
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

        define_two_op_trait!($ty, Div, sdiv, div);
        define_two_assign_op_trait!($ty, DivAssign, udiv, div_assign);

        define_two_op_trait!($ty, Rem, smod, rem);
        define_two_assign_op_trait!($ty, RemAssign, umod, rem_assign);

        define_two_op_trait!($ty, Shr, sar, shr);
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

define_sint!(S8, Bytes1, U8);
define_sint!(S16, Bytes2, U16);
define_sint!(S24, Bytes3, U24);
define_sint!(S32, Bytes4, U32);
define_sint!(S40, Bytes5, U40);
define_sint!(S48, Bytes6, U48);
define_sint!(S56, Bytes7, U56);
define_sint!(S64, Bytes8, U64);
define_sint!(S72, Bytes9, U72);
define_sint!(S80, Bytes10, U80);
define_sint!(S88, Bytes11, U88);
define_sint!(S96, Bytes12, U96);
define_sint!(S104, Bytes13, U104);
define_sint!(S112, Bytes14, U112);
define_sint!(S120, Bytes15, U120);
define_sint!(S128, Bytes16, U128);
define_sint!(S136, Bytes17, U136);
define_sint!(S144, Bytes18, U144);
define_sint!(S152, Bytes19, U152);
define_sint!(S160, Bytes20, U160);
define_sint!(S168, Bytes21, U168);
define_sint!(S176, Bytes22, U176);
define_sint!(S184, Bytes23, U184);
define_sint!(S192, Bytes24, U192);
define_sint!(S200, Bytes25, U200);
define_sint!(S208, Bytes26, U208);
define_sint!(S216, Bytes27, U216);
define_sint!(S224, Bytes28, U224);
define_sint!(S232, Bytes29, U232);
define_sint!(S240, Bytes30, U240);
define_sint!(S248, Bytes31, U248);
define_sint!(S256, Bytes32, U256);
