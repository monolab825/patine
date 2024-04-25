use core::{ptr, slice};

use crate::ffi;

const STACK_BASE: usize = 0x40;
const STACK_START: u64 = 0x60;

#[inline(never)]
#[no_mangle]
fn __yul_allocate(len: usize) -> *mut u8 {
    let stack_base = ptr::without_provenance(STACK_BASE);
    let stack_base_mut = ptr::without_provenance_mut(STACK_BASE);
    let ptr = unsafe { ffi::__yul_mload(stack_base) };

    let ptr = if unsafe { ffi::__yul_iszero(ptr) } {
        STACK_START
    } else {
        ptr
    };

    unsafe { ffi::__yul_mstore(stack_base_mut, ffi::__yul_add(ptr, len as u64)) };

    ptr::without_provenance_mut(ptr as usize)
}

#[inline]
pub fn allocate<'a>(len: usize) -> &'a mut [u8] {
    let ptr = __yul_allocate(len);
    unsafe { slice::from_raw_parts_mut(ptr, len) }
}

#[inline]
pub fn deallocate(len: usize) {
    let stack_base = ptr::without_provenance(STACK_BASE);
    let stack_base_mut = ptr::without_provenance_mut(STACK_BASE);

    let current = unsafe { ffi::__yul_mload(stack_base) };
    let next = unsafe { ffi::__yul_sub(current, len as u64) };

    unsafe { ffi::__yul_mstore(stack_base_mut, next) };
}
