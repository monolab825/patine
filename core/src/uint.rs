use core::ops::{Add, Div, Mul, Sub};

use crate::{builtin, Bytes32};

#[repr(C)]
pub struct U256(pub(crate) u32);

impl From<u32> for U256 {
    #[inline]
    fn from(value: u32) -> Self {
        let r = unsafe { builtin::__yul__ext_literal(0, 0, 0, 0, 0, 0, 0, value) };

        Self(r)
    }
}

impl From<Bytes32> for U256 {
    fn from(value: Bytes32) -> Self {
        Self(value.0)
    }
}

macro_rules! defined_uint_ops {
    ($s:ty, $t:ty, $f:ident, $n:tt) => {
        impl $t for $s {
            type Output = Self;

            fn $f(self, rhs: Self) -> Self::Output {
                let o = self.0 $n rhs.0;
                Self(o)
            }
        }
    };
}

defined_uint_ops!(U256, Add, add, +);
defined_uint_ops!(U256, Sub, sub, -);
defined_uint_ops!(U256, Mul, mul, *);
defined_uint_ops!(U256, Div, div, /);
