use patine_core::{builtin, Address, U256};

use crate::Context;

pub trait Contract {
    fn new(address: Address) -> Self;

    fn selfaddress(&self) -> Option<&Address>;

    fn address(&self) -> Address {
        match self.selfaddress() {
            Some(addr) => addr.clone(),
            None => builtin::address(),
        }
    }

    fn balance(&self) -> U256 {
        match self.selfaddress() {
            Some(addr) => builtin::balance(addr.clone()),
            None => builtin::selfbalance(),
        }
    }

    // fn code(&self) -> impl Data {}

    fn context(&self) -> Context {
        Context {}
    }
}
