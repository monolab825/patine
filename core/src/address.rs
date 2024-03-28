// use crate::{builtin, Bytes32};

use crate::builtin::Cnt;

#[repr(C)]
pub struct Address(pub(crate) Cnt);

// impl From<Bytes32> for Address {
//     fn from(value: Bytes32) -> Self {
//         let mask = unsafe { builtin::__yul__ext_literal(0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0xff) };
//
//         Self(mask & value.0)
//     }
// }
