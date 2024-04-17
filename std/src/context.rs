use patine_core::{builtin, Address, U256};

pub struct Context {}

impl Context {
    pub fn gas(&self) -> U256 {
        builtin::gas()
    }

    pub fn caller(&self) -> Address {
        builtin::caller()
    }

    pub fn value(&self) -> U256 {
        builtin::callvalue()
    }

    // pub fn calldata(&self) -> impl Data {}

    // pub fn return_data(&self) -> impl Data {}
}
