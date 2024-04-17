use patine_core::{builtin, Address, U256};

pub struct Transaction {}

impl Transaction {
    pub fn origin() -> Address {
        builtin::origin()
    }

    pub fn gasprice() -> U256 {
        builtin::gas()
    }
}
