#![no_std]

use patine_core::builtin::{
    caller, callvalue, datacopy, dataoffset, datasize, iszero, ret, sstore,
};
use patine_std::{
    builtin::{calldataload, revert, sload},
    require, selector, uint, U256,
};

#[no_mangle]
pub extern "C" fn erc20() {
    sstore(0.into(), caller());

    let mut code = [0u8; datasize(erc20_deployed)];
    let offset = dataoffset(erc20_deployed);

    datacopy(&mut code, offset);

    ret(&code)
}

fn retrieve() {
    let v = sload(uint!(0));
}

fn store() {
    let v = calldataload(uint!(4));

    sstore(uint!(0), v)
}

pub extern "C" fn erc20_deployed() {
    require(iszero(callvalue()));

    match selector().unbox() {
        0x2e64cec1 => retrieve(),
        0x6057361d => store(),
        _ => revert(&[]),
    }
}
