use crate::{builtin, Bytes32};

pub fn load(key: &Bytes32) -> Bytes32 {
    let r = unsafe { builtin::__yul_sload(key.0) };

    Bytes32(r)
}

pub fn store(key: Bytes32, value: Bytes32) {
    unsafe { builtin::__yul_sstore(key.0, value.0) }
}
