use core::ops::{Add, Div, Mul, Shr, Sub};

use crate::{builtin::Cnt, Bytes32};

extern "C" {
    fn __yul__ext_literal(x0: Cnt, x1: Cnt, x2: Cnt, x3: Cnt) -> Cnt;
}

#[repr(C)]
pub struct U256(pub(crate) Cnt);

impl U256 {
    pub fn from_raw(x0: u64, x1: u64, x2: u64, x3: u64) -> Self {
        U256(unsafe { __yul__ext_literal(x0, x1, x2, x3) })
    }
}

impl From<u32> for U256 {
    #[inline]
    fn from(value: u32) -> Self {
        let r = unsafe { __yul__ext_literal(0, 0, 0, value as Cnt) };

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

impl Shr<u8> for U256 {
    type Output = U256;

    fn shr(self, rhs: u8) -> Self::Output {
        Self(self.0 >> rhs)
    }
}
