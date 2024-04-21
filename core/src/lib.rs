#![no_std]
#![feature(exposed_provenance)]

pub mod builtin;
pub mod ffi;

mod prelude;
pub use prelude::*;

mod uint;
pub use uint::*;

mod sint;
pub use sint::*;

mod bytes;
pub use bytes::*;

mod address;
pub use address::*;

mod alloc;
pub use alloc::*;

#[macro_export]
macro_rules! define_two_op_trait {
    ($s:ty, $t:ty, $f:ident, $c: ident) => {
        impl $t for $s {
            type Output = Self;

            #[inline]
            fn $c(self, rhs: Self) -> Self::Output {
                builtin::$f(self, rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! define_two_assign_op_trait {
    ($s:ty, $t:ty, $f:ident, $c: ident) => {
        impl $t for $s {
            fn $c(&mut self, rhs: Self) {
                *self = builtin::$f(self.clone(), rhs);
            }
        }
    };
}
