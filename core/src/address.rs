use crate::{ffi::NativeType, *};

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Address(pub(crate) NativeType);

impl AsNativeType for Address {
    fn as_native_type(&self) -> NativeType {
        self.0
    }
}

impl FromNativeType for Address {
    fn from_native_type(x: NativeType) -> Self {
        Self(x)
    }
}

macro_rules! define_address_from {
    ($($ft:ty),*) => {
        $(
            impl From<$ft> for Address {
                fn from(value: $ft) -> Self {
                    Self(value.0)
                }
            }
        )*
    };
}

define_address_from!(
    Bytes1, U8, S8, Bytes2, U16, S16, Bytes3, U24, S24, Bytes4, U32, S32, Bytes5, U40, S40, Bytes6,
    U48, S48, Bytes7, U56, S56, Bytes8, U64, S64, Bytes9, U72, S72, Bytes10, U80, S80, Bytes11,
    U88, S88, Bytes12, U96, S96, Bytes13, U104, S104, Bytes14, U112, S112, Bytes15, U120, S120,
    Bytes16, U128, S128, Bytes17, U136, S136, Bytes18, U144, S144, Bytes19, U152, S152, Bytes20,
    U160, S160
);
