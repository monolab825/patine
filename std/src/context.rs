use patine_core::{builtin, Address, U256};

use crate::data::{Calldata, Data};

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

    pub fn calldata(&self) -> impl Data {
        Calldata {}
    }

    // pub fn return_data(&self) -> impl Data {}
}
