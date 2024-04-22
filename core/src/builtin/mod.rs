pub mod number;
pub use number::*;

pub mod keccak;
pub use keccak::*;

pub mod vm;
pub use vm::*;

pub mod memory;
pub use memory::*;

pub mod storage;
pub use storage::*;

pub mod msg;
pub use msg::*;

pub mod account;
pub use account::*;

pub mod call;
pub use call::*;

pub mod event;
pub use event::*;

pub mod block;
pub use block::*;

pub mod transaction;
pub use transaction::*;

pub mod data;
pub use data::*;

#[macro_export]
macro_rules! define_two_op {
    ($op:ident, $f:ident) => {
        #[inline]
        pub fn $op<R: FromNativeType>(x: impl AsNativeType, y: impl AsNativeType) -> R {
            R::from_native_type(unsafe { ffi::$f(x.as_native_type(), y.as_native_type()) })
        }
    };
}

#[macro_export]
macro_rules! define_two_op_native_return {
    ($op:ident, $f:ident, $r:ty) => {
        #[inline]
        pub fn $op(x: impl AsNativeType, y: impl AsNativeType) -> $r {
            unsafe { ffi::$f(x.as_native_type(), y.as_native_type()) }
        }
    };
}

#[macro_export]
macro_rules! define_one {
    ($op:ident, $x:ty, $r:ident, $f:ident) => {
        #[inline]
        pub fn $op(x: $x) -> $r {
            $r(unsafe { ffi::$f(x.0) })
        }
    };
}

#[macro_export]
macro_rules! define_zero {
    ($op:ident, $r:ident, $f:ident) => {
        #[inline]
        pub fn $op() -> $r {
            $r(unsafe { ffi::$f() })
        }
    };
}
