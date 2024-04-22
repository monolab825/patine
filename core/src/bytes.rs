use core::{
    cmp::{Eq, Ord, PartialEq, PartialOrd},
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Rem, RemAssign, Shl,
        ShlAssign, Shr, ShrAssign,
    },
};

use crate::{ffi::NativeType, *};

macro_rules! define_fixed_bytes {
    ($ty:ident, $($ft:ty),*) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        #[repr(C)]
        #[repr(align(32))]
        pub struct $ty(pub(crate) NativeType);

        impl $ty {
            // TODO: use U8
            pub fn get(&self, idx: U256) -> U256 {
                builtin::byte(*self, idx)
            }
        }

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

        impl FixedBytes for $ty {}

        $(
            impl From<$ft> for $ty {
                fn from(value: $ft) -> Self {
                    Self(value.0)
                }
            }
        )*


        define_two_op_trait!($ty, BitAnd, bitand, bitand);
        define_two_assign_op_trait!($ty, BitAndAssign, bitand, bitand_assign);

        define_two_op_trait!($ty, BitOr, bitor, bitor);
        define_two_assign_op_trait!($ty, BitOrAssign, bitor, bitor_assign);

        define_two_op_trait!($ty, BitXor, bitxor, bitxor);
        define_two_assign_op_trait!($ty, BitXorAssign, bitxor, bitxor_assign);

        impl Not for $ty {
            type Output = $ty;

            fn not(self) -> Self::Output {
                builtin::not(self)
            }
        }

        define_two_op_trait!($ty, Rem, umod, rem);
        define_two_assign_op_trait!($ty, RemAssign, umod, rem_assign);

        define_two_op_trait!($ty, Shl, shl, shl);
        define_two_assign_op_trait!($ty, ShlAssign, shl, shl_assign);

        define_two_op_trait!($ty, Shr, shr, shr);
        define_two_assign_op_trait!($ty, ShrAssign, shr, shr_assign);
    };
}

define_fixed_bytes!(Bytes1, U8, S8);
define_fixed_bytes!(Bytes2, U16, S16);
define_fixed_bytes!(Bytes3, U24, S24);
define_fixed_bytes!(Bytes4, U32, S32);
define_fixed_bytes!(Bytes5, U40, S40);
define_fixed_bytes!(Bytes6, U48, S48);
define_fixed_bytes!(Bytes7, U56, S56);
define_fixed_bytes!(Bytes8, U64, S64);
define_fixed_bytes!(Bytes9, U72, S72);
define_fixed_bytes!(Bytes10, U80, S80);
define_fixed_bytes!(Bytes11, U88, S88);
define_fixed_bytes!(Bytes12, U96, S96);
define_fixed_bytes!(Bytes13, U104, S104);
define_fixed_bytes!(Bytes14, U112, S112);
define_fixed_bytes!(Bytes15, U120, S120);
define_fixed_bytes!(Bytes16, U128, S128);
define_fixed_bytes!(Bytes17, U136, S136);
define_fixed_bytes!(Bytes18, U144, S144);
define_fixed_bytes!(Bytes19, U152, S152);
define_fixed_bytes!(Bytes20, U160, S160);
define_fixed_bytes!(Bytes21, U168, S168);
define_fixed_bytes!(Bytes22, U176, S176);
define_fixed_bytes!(Bytes23, U184, S184);
define_fixed_bytes!(Bytes24, U192, S192);
define_fixed_bytes!(Bytes25, U200, S200);
define_fixed_bytes!(Bytes26, U208, S208);
define_fixed_bytes!(Bytes27, U216, S216);
define_fixed_bytes!(Bytes28, U224, S224);
define_fixed_bytes!(Bytes29, U232, S232);
define_fixed_bytes!(Bytes30, U240, S240);
define_fixed_bytes!(Bytes31, U248, S248);
define_fixed_bytes!(Bytes32, U256, S256);
