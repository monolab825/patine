use crate::{ffi::NativeType, AsNativeType, FromNativeType, Integer};

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

impl Integer for S256 {}
