use patine_core::builtin::revert;

#[inline]
pub fn require(cond: bool) {
    if cond {
        revert(&[])
    }
}
