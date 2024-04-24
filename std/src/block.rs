use patine_core::{builtin, Address, Bytes32, U256};

pub struct Block {}

impl Block {
    pub fn chain_id() -> U256 {
        builtin::chainid()
    }

    pub fn base_fee() -> U256 {
        builtin::basefee()
    }

    pub fn blob_base_fee() -> U256 {
        builtin::blobbasefee()
    }

    pub fn block_hash(number: U256) -> Bytes32 {
        builtin::blockhash(number)
    }

    pub fn coinbase() -> Address {
        builtin::coinbase()
    }

    pub fn timestamp() -> U256 {
        builtin::timestamp()
    }

    pub fn number() -> U256 {
        builtin::number()
    }

    pub fn difficulty() -> U256 {
        builtin::difficulty()
    }

    pub fn prevrandao() -> Bytes32 {
        builtin::prevrandao()
    }

    pub fn gaslimit() -> U256 {
        builtin::gaslimit()
    }
}
