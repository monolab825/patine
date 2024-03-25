extern "C" {
    pub fn __yul__ext_literal(
        x0: u32,
        x1: u32,
        x2: u32,
        x3: u32,
        x4: u32,
        x5: u32,
        x6: u32,
        x7: u32,
    ) -> u32;

    pub fn __yul_keccak256(ptr: *const u8, len: usize) -> u32;

    pub fn __yul_sload(p: u32) -> u32;
    pub fn __yul_sstore(p: u32, v: u32);

    // Context
    pub fn __yul_gas() -> u32;
    pub fn __yul_address() -> u32;
    pub fn __yul_caller() -> u32;
    pub fn __yul_callvalue() -> u32;
    pub fn __yul_calldataload(p: u32) -> u32;
    pub fn __yul_calldatasize() -> u32;
    pub fn __yul_calldatacopy(t: usize, f: usize, s: usize);
}
