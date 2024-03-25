use crate::{builtin, Address};

/// Contract call from other contract of EOA
pub struct Call;

impl Call {
    pub fn address(&self) -> Address {
        Address(unsafe { builtin::__yul_address() })
    }

    pub fn sender(&self) -> Address {
        Address(unsafe { builtin::__yul_caller() })
    }
}
