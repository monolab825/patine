#![no_std]

use patine_core::builtin::{
    caller, callvalue, datacopy, dataoffset, datasize, iszero, ret, sstore,
};
use patine_std::{
    builtin::{calldataload, mstore, revert, sload},
    require, selector, uint,
};

#[no_mangle]
pub extern "C" fn _store() {
    sstore(0.into(), caller());

    let mut code = [0u8; datasize(_store_deployed)];
    let offset = dataoffset(_store_deployed);

    datacopy(&mut code, offset);

    ret(&code)
}

fn retrieve() {
    let v = sload(uint!(0));

    let mut ret_arr = [0u8; 32];

    mstore(v, &mut ret_arr);
    ret(&ret_arr);
}

fn store() {
    let v = calldataload(uint!(4));

    sstore(uint!(0), v)
}

#[no_mangle]
pub extern "C" fn _store_deployed() {
    require(iszero(callvalue()));

    match selector().unbox() {
        0x2e64cec1 => retrieve(),
        0x6057361d => store(),
        _ => revert(&[]),
    }
}
