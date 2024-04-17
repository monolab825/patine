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
//
// mod alloc;
// pub use alloc::*;
