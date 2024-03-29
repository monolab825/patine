use patine_core::builtin::revert_null;

#[inline]
pub fn require(cond: bool) {
    if cond {
        revert_null()
    }
}
