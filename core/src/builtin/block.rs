use crate::{ffi, Address, AsNativeType, Bytes32, U256};

#[inline]
pub fn chainid() -> U256 {
    U256(unsafe { ffi::__yul_chainid() })
}

#[inline]
pub fn basefee() -> U256 {
    U256(unsafe { ffi::__yul_basefee() })
}

#[inline]
pub fn blobbasefee() -> U256 {
    U256(unsafe { ffi::__yul_blobbasefee() })
}

#[inline]
pub fn blockhash(b: U256) -> Bytes32 {
    Bytes32(unsafe { ffi::__yul_blockhash(b.as_native_type()) })
}

#[inline]
pub fn coinbase() -> Address {
    Address(unsafe { ffi::__yul_coinbase() })
}

#[inline]
pub fn timestamp() -> Address {
    Address(unsafe { ffi::__yul_timestamp() })
}

#[inline]
pub fn number() -> Address {
    Address(unsafe { ffi::__yul_number() })
}

#[inline]
pub fn difficulty() -> Address {
    Address(unsafe { ffi::__yul_difficulty() })
}

#[inline]
pub fn prevrandao() -> Address {
    Address(unsafe { ffi::__yul_prevrandao() })
}

#[inline]
pub fn gaslimit() -> Address {
    Address(unsafe { ffi::__yul_gaslimit() })
}
