use crate::{ffi, Address, AsNativeType, Bytes32, U256};

#[inline]
pub fn origin() -> Address {
    Address(unsafe { ffi::__yul_origin() })
}

#[inline]
pub fn gasprice() -> U256 {
    U256(unsafe { ffi::__yul_gasprice() })
}

#[inline]
pub fn blobhash(b: U256) -> Bytes32 {
    Bytes32(unsafe { ffi::__yul_blockhash(b.as_native_type()) })
}
