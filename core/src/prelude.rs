use crate::ffi::NativeType;

pub trait AsNativeType {
    fn as_native_type(&self) -> NativeType;
}

pub trait FromNativeType {
    fn from_native_type(x: NativeType) -> Self;
}
