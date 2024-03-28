use crate::U256;

#[repr(C)]
pub struct Bytes32(pub(crate) u32);

impl From<U256> for Bytes32 {
    fn from(value: U256) -> Self {
        Self(value.0)
    }
}
