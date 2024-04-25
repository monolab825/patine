use patine_core::{builtin, U256};

pub trait StorageBackend {
    fn store(key: U256, value: U256);

    fn load(key: U256) -> U256;
}

pub struct Storage {}

impl StorageBackend for Storage {
    fn load(key: U256) -> U256 {
        builtin::sload(key)
    }

    fn store(key: U256, value: U256) {
        builtin::sstore(key, value)
    }
}

pub struct TransientStorage {}

impl StorageBackend for TransientStorage {
    fn store(key: U256, value: U256) {
        builtin::tstore(key, value)
    }

    fn load(key: U256) -> U256 {
        builtin::tload(key)
    }
}
