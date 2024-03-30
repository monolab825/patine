use crate::U256;

use super::Cnt;

extern "C" {
    pub(crate) fn __yul_iszero(x: Cnt) -> bool;
    fn __yul_exp(x: Cnt, y: Cnt) -> Cnt;
    fn __yul_byte(n: Cnt, x: Cnt) -> Cnt;
    fn __yul_addmod(x: Cnt, y: Cnt, m: Cnt) -> Cnt;
    fn __yul_mulmod(x: Cnt, y: Cnt, m: Cnt) -> Cnt;

}

#[inline]
pub fn iszero(x: U256) -> bool {
    unsafe { __yul_iszero(x.0) }
}

#[inline]
pub fn exp(x: U256, y: U256) -> U256 {
    U256(unsafe { __yul_exp(x.0, y.0) })
}

#[inline]
pub fn byte(n: U256, x: U256) -> U256 {
    U256(unsafe { __yul_byte(n.0, x.0) })
}

#[inline]
pub fn addmod(x: U256, y: U256, m: U256) -> U256 {
    U256(unsafe { __yul_addmod(x.0, y.0, m.0) })
}

#[inline]
pub fn mulmod(x: U256, y: U256, m: U256) -> U256 {
    U256(unsafe { __yul_mulmod(x.0, y.0, m.0) })
}
