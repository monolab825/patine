#![no_std]
#![feature(strict_provenance)]

pub mod builtin;
pub mod ffi;

mod prelude;
pub use prelude::*;

pub mod uint;
pub use uint::*;

pub mod sint;
pub use sint::*;

pub mod bytes;
pub use bytes::*;

pub mod address;
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
