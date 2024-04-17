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

#[macro_export]
macro_rules! define_two_op {
    ($op:ident, $x:ty, $y:ty, $r:ident, $f:ident) => {
        #[inline]
        pub fn $op(x: $x, y: $y) -> $r {
            $r(unsafe { ffi::$f(x.0, y.0) })
        }
    };
}

#[macro_export]
macro_rules! define_two_noreturn_op {
    ($op:ident, $x:ty, $y:ty, $f:ident) => {
        #[inline]
        pub fn $op(x: $x, y: $y) {
            unsafe { ffi::$f(x.0, y.0) }
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
