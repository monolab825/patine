use patine_core::{allocate, builtin, deallocate, Address, FromNativeType};

pub trait Data {
    fn size(&self) -> usize;

    fn copy(&self, offset: usize, target: &mut [u8]);

    fn load<R>(&self, offset: usize) -> R
    where
        R: FromNativeType,
    {
        let res = allocate(32);

        self.copy(offset, res);

        deallocate(32);

        builtin::mload(res)
    }
}

#[derive(Default)]
pub struct Code {
    addr: Option<Address>,
}

impl Code {
    pub fn new(addr: Address) -> Self {
        Self { addr: Some(addr) }
    }
}

impl Data for Code {
    fn size(&self) -> usize {
        match self.addr {
            Some(addr) => builtin::extcodesize(addr),
            None => builtin::codesize(),
        }
    }

    fn copy(&self, offset: usize, target: &mut [u8]) {
        match self.addr {
            Some(a) => builtin::extcodecopy(a, target, offset),
            None => builtin::codecopy(target, offset),
        }
    }
}

pub struct Calldata {}

impl Data for Calldata {
    fn size(&self) -> usize {
        builtin::calldatasize()
    }

    fn copy(&self, offset: usize, target: &mut [u8]) {
        builtin::calldatacopy(target, offset)
    }

    fn load<R>(&self, offset: usize) -> R
    where
        R: FromNativeType,
    {
        builtin::calldataload(offset)
    }
}

pub struct ReturnData {}

impl Data for ReturnData {
    fn size(&self) -> usize {
        builtin::returndatasize()
    }

    fn copy(&self, offset: usize, target: &mut [u8]) {
        builtin::returndatacopy(target, offset)
    }
}
