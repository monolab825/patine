use patine_core::builtin::revert;

pub fn require(cond: bool) {
    if cond {
        revert(&[])
    }
}
