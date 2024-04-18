pub type NativeType = u64;

extern "C" {
    /// Magic function to build literal number
    pub fn __yul__ext_literal(
        x0: NativeType,
        x1: NativeType,
        x2: NativeType,
        x3: NativeType,
    ) -> NativeType;
    /// stop execution, identical to return(0, 0)
    pub fn __yul_stop() -> NativeType;
    /// x + y
    pub fn __yul_add(x: NativeType, y: NativeType) -> NativeType;
    /// x - y
    pub fn __yul_sub(x: NativeType, y: NativeType) -> NativeType;
    /// x * y
    pub fn __yul_mul(x: NativeType, y: NativeType) -> NativeType;
    /// x / y or 0 if y == 0
    pub fn __yul_div(x: NativeType, y: NativeType) -> NativeType;
    /// x / y, for signed numbers in two’s complement, 0 if y == 0
    pub fn __yul_sdiv(x: NativeType, y: NativeType) -> NativeType;
    /// x % y, 0 if y == 0
    pub fn __yul_mod(x: NativeType, y: NativeType) -> NativeType;
    /// x % y, for signed numbers in two’s complement, 0 if y == 0
    pub fn __yul_smod(x: NativeType, y: NativeType) -> NativeType;
    /// x to the power of y
    pub fn __yul_exp(x: NativeType, y: NativeType) -> NativeType;
    /// bitwise “not” of x (every bit of x is negated)
    pub fn __yul_not(x: NativeType) -> NativeType;
    /// 1 if x < y, 0 otherwise
    pub fn __yul_lt(x: NativeType, y: NativeType) -> NativeType;
    /// 1 if x > y, 0 otherwise
    pub fn __yul_gt(x: NativeType, y: NativeType) -> NativeType;
    /// 1 if x < y, 0 otherwise, for signed numbers in two’s complement
    pub fn __yul_slt(x: NativeType, y: NativeType) -> NativeType;
    /// 1 if x > y, 0 otherwise, for signed numbers in two’s complement
    pub fn __yul_sgt(x: NativeType, y: NativeType) -> NativeType;
    /// 1 if x == y, 0 otherwise
    pub fn __yul_eq(x: NativeType, y: NativeType) -> NativeType;
    /// 1 if x == 0, 0 otherwise
    pub fn __yul_iszero(x: NativeType) -> bool;
    /// bitwise “and” of x and y
    pub fn __yul_and(x: NativeType, y: NativeType) -> NativeType;
    /// bitwise “or” of x and y
    pub fn __yul_or(x: NativeType, y: NativeType) -> NativeType;
    /// bitwise “xor” of x and y
    pub fn __yul_xor(x: NativeType, y: NativeType) -> NativeType;
    /// nth byte of x, where the most significant byte is the 0th byte
    pub fn __yul_byte(n: NativeType, x: NativeType) -> NativeType;
    /// logical shift left y by x bits
    pub fn __yul_shl(x: NativeType, y: NativeType) -> NativeType;
    /// logical shift right y by x bits
    pub fn __yul_shr(x: NativeType, y: NativeType) -> NativeType;
    /// signed arithmetic shift right y by x bits
    pub fn __yul_sar(x: NativeType, y: NativeType) -> NativeType;
    /// (x + y) % m with arbitrary precision arithmetic, 0 if m == 0
    pub fn __yul_addmod(x: NativeType, y: NativeType, m: NativeType) -> NativeType;
    /// (x * y) % m with arbitrary precision arithmetic, 0 if m == 0
    pub fn __yul_mulmod(x: NativeType, y: NativeType, m: NativeType) -> NativeType;
    /// sign extend from (i*8+7)th bit counting from least significant
    pub fn __yul_signextend(i: NativeType, x: NativeType) -> NativeType;
    /// keccak(mem[p…(p+n)))
    pub fn __yul_keccak256(p: *const u8, n: usize) -> NativeType;
    /// current position in code
    pub fn __yul_pc() -> NativeType;
    /// discard value x
    pub fn __yul_pop(x: NativeType);
    /// mem[p…(p+32))
    pub fn __yul_mload(p: *const u8) -> NativeType;
    /// mem[p…(p+32)) := v
    pub fn __yul_mstore(p: *mut u8, v: NativeType);
    /// mem\[p\] := v & 0xff (only modifies a single byte)
    pub fn __yul_mstore8(p: *mut u8, v: NativeType);
    /// storage\[p\]
    pub fn __yul_sload(p: NativeType) -> NativeType;
    /// storage\[p\] := v
    pub fn __yul_sstore(p: NativeType, v: NativeType);
    /// transientStorage\[p\]
    pub fn __yul_tload(p: NativeType) -> NativeType;
    /// transientStorage\[p\] := v
    pub fn __yul_tstore(p: NativeType, v: NativeType);
    /// size of memory, i.e. largest accessed memory index
    pub fn __yul_msize() -> usize;
    /// gas still available to execution
    pub fn __yul_gas() -> NativeType;
    /// address of the current contract / execution context
    pub fn __yul_address() -> NativeType;
    /// wei balance at address a
    pub fn __yul_balance(a: NativeType) -> NativeType;
    /// equivalent to balance(address()), but cheaper
    pub fn __yul_selfbalance() -> NativeType;
    /// call sender (excluding
    pub fn __yul_caller() -> NativeType;
    /// wei sent together with the current call
    pub fn __yul_callvalue() -> NativeType;
    /// call data starting from position p (32 bytes)
    pub fn __yul_calldataload(p: NativeType) -> NativeType;
    /// size of call data in bytes
    pub fn __yul_calldatasize() -> usize;
    /// copy s bytes from calldata at position f to mem at position t
    pub fn __yul_calldatacopy(t: *mut u8, f: usize, s: usize);
    /// size of the code of the current contract / execution context
    pub fn __yul_codesize() -> usize;
    /// copy s bytes from code at position f to mem at position t
    pub fn __yul_codecopy(t: *mut u8, f: usize, s: usize);
    /// size of the code at address a
    pub fn __yul_extcodesize(a: NativeType) -> usize;
    /// like codecopy(t, f, s) but take code at address a
    pub fn __yul_extcodecopy(a: NativeType, t: *mut u8, f: usize, s: usize);
    /// size of the last returndata
    pub fn __yul_returndatasize() -> usize;
    /// copy s bytes from returndata at position f to mem at position t
    pub fn __yul_returndatacopy(t: *mut u8, f: usize, s: usize);
    /// copy s bytes from mem at position f to mem at position t
    pub fn __yul_mcopy(t: *mut u8, f: *const u8, s: usize);
    /// code hash of address a
    pub fn __yul_extcodehash(a: NativeType) -> NativeType;
    /// create new contract with code mem[p…(p+n)) and send v wei and return the new address; returns 0 on error
    pub fn __yul_create(v: NativeType, p: NativeType, n: NativeType) -> NativeType;
    /// create new contract with code mem[p…(p+n)) at address keccak256(0xff . this . s . keccak256(mem[p…(p+n))) and send v wei and return the new address, where
    pub fn __yul_create2(v: NativeType, p: NativeType, n: NativeType, s: NativeType) -> NativeType;
    /// call contract at address a with input mem[in…(in+insize)) providing g gas and v wei and output area mem[out…(out+outsize)) returning 0 on error (eg. out of gas) and 1 on success
    pub fn __yul_call(
        g: NativeType,
        a: NativeType,
        v: NativeType,
        input: NativeType,
        insize: NativeType,
        out: NativeType,
        outsize: NativeType,
    ) -> NativeType;
    /// identical to
    pub fn __yul_callcode(
        g: NativeType,
        a: NativeType,
        v: NativeType,
        input: NativeType,
        insize: NativeType,
        out: NativeType,
        outsize: NativeType,
    ) -> NativeType;
    /// identical to
    pub fn __yul_delegatecall(
        g: NativeType,
        a: NativeType,
        input: NativeType,
        insize: NativeType,
        out: NativeType,
        outsize: NativeType,
    ) -> NativeType;
    /// identical to
    pub fn __yul_staticcall(
        g: NativeType,
        a: NativeType,
        input: NativeType,
        insize: NativeType,
        out: NativeType,
        outsize: NativeType,
    ) -> NativeType;
    /// end execution, return data mem[p…(p+s))
    pub fn __yul_return(p: NativeType, s: NativeType) -> NativeType;
    /// end execution, revert state changes, return data mem[p…(p+s))
    pub fn __yul_revert(p: NativeType, s: NativeType) -> NativeType;
    /// end execution, destroy current contract and send funds to a (deprecated)
    pub fn __yul_selfdestruct(a: NativeType) -> NativeType;
    /// end execution with invalid instruction
    pub fn __yul_invalid() -> NativeType;
    /// log data mem[p…(p+s))
    pub fn __yul_log0(p: NativeType, s: NativeType) -> NativeType;
    /// log data mem[p…(p+s)) with topic t1
    pub fn __yul_log1(p: NativeType, s: NativeType, t1: NativeType) -> NativeType;
    /// log data mem[p…(p+s)) with topics t1, t2
    pub fn __yul_log2(p: NativeType, s: NativeType, t1: NativeType, t2: NativeType) -> NativeType;
    /// log data mem[p…(p+s)) with topics t1, t2, t3
    pub fn __yul_log3(
        p: NativeType,
        s: NativeType,
        t1: NativeType,
        t2: NativeType,
        t3: NativeType,
    ) -> NativeType;
    /// log data mem[p…(p+s)) with topics t1, t2, t3, t4
    pub fn __yul_log4(
        p: NativeType,
        s: NativeType,
        t1: NativeType,
        t2: NativeType,
        t3: NativeType,
        t4: NativeType,
    ) -> NativeType;
    /// ID of the executing chain (EIP-1344)
    pub fn __yul_chainid() -> NativeType;
    /// current block’s base fee (EIP-3198 and EIP-1559)
    pub fn __yul_basefee() -> NativeType;
    /// current block’s blob base fee (EIP-7516 and EIP-4844)
    pub fn __yul_blobbasefee() -> NativeType;
    /// transaction sender
    pub fn __yul_origin() -> NativeType;
    /// gas price of the transaction
    pub fn __yul_gasprice() -> NativeType;
    /// hash of block nr b - only for last 256 blocks excluding current
    pub fn __yul_blockhash(b: NativeType) -> NativeType;
    /// versioned hash of transaction’s i-th blob
    pub fn __yul_blobhash(i: NativeType) -> NativeType;
    /// current mining beneficiary
    pub fn __yul_coinbase() -> NativeType;
    /// timestamp of the current block in seconds since the epoch
    pub fn __yul_timestamp() -> NativeType;
    /// current block number
    pub fn __yul_number() -> NativeType;
    /// difficulty of the current block (see note below)
    pub fn __yul_difficulty() -> NativeType;
    /// randomness provided by the beacon chain (see note below)
    pub fn __yul_prevrandao() -> NativeType;
    /// block gas limit of the current block
    pub fn __yul_gaslimit() -> NativeType;
}
