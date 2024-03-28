use crate::U256;

pub type EntryFunction = extern "C" fn();

extern "C" {
    fn __yul_datasize(f: EntryFunction) -> u32;
    fn __yul_dataoffset(f: EntryFunction) -> u32;
    fn __yul_datacopy(t: *mut u8, f: u32, l: usize);
}

pub fn datasize(func: EntryFunction) -> U256 {
    U256(unsafe { __yul_datasize(func) })
}

pub fn dataoffset(func: EntryFunction) -> U256 {
    U256(unsafe { __yul_dataoffset(func) })
}

pub fn datacopy(target: &mut [u8], offset: U256) {
    unsafe { __yul_datacopy(target.as_mut_ptr(), offset.0, target.len()) }
}
