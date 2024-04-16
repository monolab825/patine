use patine_core::U256;

pub trait StorageBackend {
    fn store(key: U256, value: U256);

    fn load(key: U256) -> U256;
}
