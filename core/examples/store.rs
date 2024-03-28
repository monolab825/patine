#![no_std]

use patine_core::builtin::{calldataload, caller, sstore};

#[no_mangle]
pub extern "C" fn erc20() {
    sstore(0.into(), caller());
}

pub extern "C" fn erc20_deployed() {}
