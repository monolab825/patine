use patine_core::{builtin::calldataload, Bytes32, Bytes4};
use patine_macros::uint;

pub fn selector() -> Bytes4 {
    let selector_word = calldataload(uint!(0));

    let res = selector_word >> uint!(224);

    Bytes4::from(Bytes32::from(res))
}
