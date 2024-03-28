#![no_std]

use patine_core::builtin::{
    caller, callvalue, datacopy, dataoffset, datasize, iszero, ret, sstore,
};
use patine_std::require;

#[no_mangle]
pub extern "C" fn erc20() {
    sstore(0.into(), caller());

    let mut code = [0u8; datasize(erc20_deployed)];
    let offset = dataoffset(erc20_deployed);

    datacopy(&mut code, offset);

    ret(&code)
}

pub extern "C" fn erc20_deployed() {
    require(iszero(callvalue()))
}
