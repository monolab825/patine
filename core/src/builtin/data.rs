use crate::ffi::{self, EntryFunction};

pub fn datacopy(f: usize, target: &mut [u8]) {
    unsafe { ffi::__yul_datacopy(target.as_mut_ptr(), f, target.len()) }
}

pub fn objectsize(f: EntryFunction) -> usize {
    unsafe { ffi::__yul_objectsize(f) }
}

pub fn objectoffset(f: EntryFunction) -> usize {
    unsafe { ffi::__yul_objectoffset(f) }
}
