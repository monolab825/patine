use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Rem, Shl, Shr, Sub};

use crate::{builtin::Cnt, Bytes32};

extern "C" {
    fn __yul__ext_literal(x0: Cnt, x1: Cnt, x2: Cnt, x3: Cnt) -> Cnt;
}

macro_rules! defined_uint_ops {
    ($s:ty, $t:ty, $f:ident) => {
        impl $t for $s {
            type Output = Self;

            fn $f(self, rhs: Self) -> Self::Output {
                let o = self.0.$f(&rhs.0);
                Self(o)
            }
        }
    };
}

macro_rules! defined_uint {
    ($s:ident) => {
        #[repr(C)]
        pub struct $s(pub(crate) Cnt);

        impl $s {
            pub fn from_raw(x0: u64, x1: u64, x2: u64, x3: u64) -> Self {
                Self(unsafe { __yul__ext_literal(x0, x1, x2, x3) })
            }
        }

        impl From<Bytes32> for $s {
            fn from(value: Bytes32) -> Self {
                Self(value.0)
            }
        }

        defined_uint_ops!($s, Add, add);
        defined_uint_ops!($s, Sub, sub);
        defined_uint_ops!($s, Mul, mul);
        defined_uint_ops!($s, Div, div);
        defined_uint_ops!($s, Rem, rem);

        defined_uint_ops!($s, Shr, shr);
        defined_uint_ops!($s, Shl, shl);

        defined_uint_ops!($s, BitAnd, bitand);
        defined_uint_ops!($s, BitOr, bitor);
        defined_uint_ops!($s, BitXor, bitxor);

        impl Not for $s {
            type Output = Self;

            fn not(self) -> Self::Output {
                Self(!self.0)
            }
        }

        impl PartialEq for $s {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }

        impl Eq for $s {}

        impl PartialOrd for $s {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $s {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }
    };
}

defined_uint!(U256);
defined_uint!(U128);
defined_uint!(U64);
defined_uint!(U32);
defined_uint!(U16);
defined_uint!(U8);
