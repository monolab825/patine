use core::usize;

pub type EntryFunction = extern "C" fn();

extern "C" {
    fn __yul_datacopy(t: *mut u8, f: usize, l: usize);

    fn __yul_datasize(_f: EntryFunction) -> usize;

    fn __yul_dataoffset(_f: EntryFunction) -> usize;
}

#[inline]
pub fn datasize(func: EntryFunction) -> usize {
    unsafe { __yul_datasize(func) }
}

#[inline]
pub fn dataoffset(func: EntryFunction) -> usize {
    unsafe { __yul_dataoffset(func) }
}

#[inline]
pub fn datacopy(target: &mut [u8], offset: usize) {
    unsafe { __yul_datacopy(target.as_mut_ptr(), offset, target.len()) }
}
