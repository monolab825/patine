use crate::{builtin::Cnt, U256};

#[repr(C)]
pub struct Bytes32(pub(crate) Cnt);

impl From<U256> for Bytes32 {
    fn from(value: U256) -> Self {
        Self(value.0)
    }
}

#[repr(C)]
pub struct Bytes4(pub(crate) Cnt);

impl Bytes4 {
    pub fn unbox(&self) -> Cnt {
        self.0
    }
}

impl From<u32> for Bytes4 {
    fn from(value: u32) -> Self {
        Self(value.into())
    }
}

impl From<Bytes32> for Bytes4 {
    fn from(value: Bytes32) -> Self {
        Self(value.0)
    }
}
