use crate::U256;

use super::Cnt;

extern "C" {
    fn __yul_sload(p: Cnt) -> Cnt;
    fn __yul_sstore(p: Cnt, v: Cnt);

    fn __yul_tload(p: Cnt) -> Cnt;
    fn __yul_tstore(p: Cnt, v: Cnt);

}

pub fn sload(position: U256) -> U256 {
    U256(unsafe { __yul_sload(position.0) })
}

pub fn sstore(position: U256, value: U256) {
    unsafe { __yul_sstore(position.0, value.0) }
}

pub fn tload(position: U256) -> U256 {
    U256(unsafe { __yul_tload(position.0) })
}

pub fn tstore(position: U256, value: U256) {
    unsafe { __yul_tstore(position.0, value.0) }
}
