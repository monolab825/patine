use crate::{ffi::NativeType, AsNativeType, FromNativeType};

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct S256(pub(crate) NativeType);

impl AsNativeType for S256 {
    fn as_native_type(&self) -> NativeType {
        self.0
    }
}

impl FromNativeType for S256 {
    fn from_native_type(x: NativeType) -> Self {
        Self(x)
    }
}

// impl From<Bytes32> for Address {
//     fn from(value: Bytes32) -> Self {
//         let mask = unsafe { builtin::__yul__ext_literal(0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0xff) };
//
//         Self(mask & value.0)
//     }
// }
