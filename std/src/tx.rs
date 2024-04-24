use patine_core::{builtin, Address, Bytes32, U256};

pub struct Transaction {}

impl Transaction {
    pub fn origin() -> Address {
        builtin::origin()
    }

    pub fn gasprice() -> U256 {
        builtin::gas()
    }

    pub fn blob_hash(index: U256) -> Bytes32 {
        builtin::blobhash(index)
    }
}
