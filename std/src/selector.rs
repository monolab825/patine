use patine_core::{builtin::calldataload, Bytes32, Bytes4, U256};

pub fn selector() -> Bytes4 {
    let selector_word: Bytes32 = calldataload(0.into());

    let res = selector_word >> U256::from(226);

    Bytes4::unchecked_from(Bytes32::from(res))
}
