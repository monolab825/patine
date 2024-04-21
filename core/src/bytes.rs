use core::ops::BitAnd;

use crate::{ffi::NativeType, AsNativeType, FixedBytes, FromNativeType, S256, U256};

#[repr(C)]
pub struct Bytes32(pub(crate) NativeType);

impl AsNativeType for Bytes32 {
    fn as_native_type(&self) -> NativeType {
        self.0
    }
}

impl FromNativeType for Bytes32 {
    fn from_native_type(x: NativeType) -> Self {
        Self(x)
    }
}

impl FixedBytes for Bytes32 {}

impl From<U256> for Bytes32 {
    fn from(value: U256) -> Self {
        Self(value.0)
    }
}

impl From<S256> for Bytes32 {
    fn from(value: S256) -> Self {
        Self(value.0)
    }
}

// impl BitAnd for Bytes32 {
//     fn bitand(self, rhs: Self) -> Self::Output {}
// }

// #[repr(C)]
// pub struct Bytes4(pub(crate) NativeType);
//
// impl Bytes4 {
//     pub fn unbox(&self) -> NativeType {
//         self.0
//     }
// }
//
// impl From<u32> for Bytes4 {
//     fn from(value: u32) -> Self {
//         Self(value.into())
//     }
// }
//
// impl From<Bytes32> for Bytes4 {
//     fn from(value: Bytes32) -> Self {
//         Self(value.0)
//     }
// }
