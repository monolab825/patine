use core::usize;

pub type EntryFunction = extern "C" fn();

extern "C" {
    fn __yul_datacopy(t: *mut u8, f: usize, l: usize);

}

/// NOTE: This function is just a placeholeder, the compiler will remove it.
#[inline(never)]
#[no_mangle]
const fn __yul_datasize(_f: EntryFunction) -> usize {
    0x1654
}

/// NOTE: This function is just a placeholeder, the compiler will remove it.
#[inline(never)]
#[no_mangle]
const fn __yul_dataoffset(_f: EntryFunction) -> usize {
    0x1653
}

#[inline]
pub const fn datasize(func: EntryFunction) -> usize {
    __yul_datasize(func)
}

#[inline]
pub fn dataoffset(func: EntryFunction) -> usize {
    __yul_dataoffset(func)
}

#[inline]
pub fn datacopy(target: &mut [u8], offset: usize) {
    unsafe { __yul_datacopy(target.as_mut_ptr(), offset, target.len()) }
}
