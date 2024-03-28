use crate::U256;

use super::Cnt;

extern "C" {
    fn __yul_log0(p: *const u8, s: usize);
    fn __yul_log1(p: *const u8, s: usize, t1: Cnt);
    fn __yul_log2(p: *const u8, s: usize, t1: Cnt, t2: Cnt);
    fn __yul_log3(p: *const u8, s: usize, t1: Cnt, t2: Cnt, t3: Cnt);
    fn __yul_log4(p: *const u8, s: usize, t1: Cnt, t2: Cnt, t3: Cnt, t4: Cnt);
}

pub fn log0(data: &[u8]) {
    unsafe { __yul_log0(data.as_ptr(), data.len()) }
}

pub fn log1(data: &[u8], t1: U256) {
    unsafe { __yul_log1(data.as_ptr(), data.len(), t1.0) }
}

pub fn log2(data: &[u8], t1: U256, t2: U256) {
    unsafe { __yul_log2(data.as_ptr(), data.len(), t1.0, t2.0) }
}

pub fn log3(data: &[u8], t1: U256, t2: U256, t3: U256) {
    unsafe { __yul_log3(data.as_ptr(), data.len(), t1.0, t2.0, t3.0) }
}

pub fn log4(data: &[u8], t1: U256, t2: U256, t3: U256, t4: U256) {
    unsafe { __yul_log4(data.as_ptr(), data.len(), t1.0, t2.0, t3.0, t4.0) }
}
