use core::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Rem, RemAssign, Shl,
    ShlAssign, Shr, ShrAssign,
};

use seq_macro::seq;

use crate::{
    builtin, define_two_assign_op_trait, define_two_op_trait, ffi::NativeType, AsNativeType,
    FixedBytes, FromNativeType, S256, U256,
};

macro_rules! define_fixed_bytes {
    ($ty:ident, $($ft:ty),*) => {
        #[derive(Clone, Copy)]
        #[repr(C)]
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
define_fixed_bytes!(Bytes32, U256, S256);
