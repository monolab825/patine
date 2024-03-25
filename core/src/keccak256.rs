use crate::{builtin, Bytes32};

pub fn keccak256(bytes: &[u8]) -> Bytes32 {
    let ptr = bytes.as_ptr();
    let len = bytes.len();

    let r = unsafe { builtin::__yul_keccak256(ptr, len) };

    Bytes32(r)
}
