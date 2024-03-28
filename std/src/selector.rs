use patine_core::{builtin::calldataload, uint, Bytes32, Bytes4};

pub fn selector() -> Bytes4 {
    let mask = uint!(0x100000000000000000000000000000000000000000000000000000000);

    let selector_word = calldataload(uint!(0));

    let res = (selector_word / mask) >> 224;

    Bytes4::from(Bytes32::from(res))
}
