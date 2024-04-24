use patine_core::{builtin, Address, U256};

use crate::{context::Context, tx::Transaction, Event};

pub trait Contract {
    fn new(address: Address) -> Self;

    fn selfaddress(&self) -> Option<&Address>;

    fn address(&self) -> Address {
        match self.selfaddress() {
            Some(addr) => *addr,
            None => builtin::address(),
        }
    }

    fn balance(&self) -> U256 {
        match self.selfaddress() {
            Some(addr) => builtin::balance(*addr),
            None => builtin::selfbalance(),
        }
    }

    // fn code(&self) -> impl Data {}

    fn context(&self) -> Context {
        Context {}
    }

    fn transaction(&self) -> Transaction {
        Transaction {}
    }

    fn emit(&self, event: impl Event) {
        event.emit()
    }
}
