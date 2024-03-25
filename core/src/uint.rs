use core::ops::{Add, Div, Mul, Sub};

#[repr(C)]
pub struct U256(u32);

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
