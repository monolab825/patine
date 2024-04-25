use patine_core::{builtin, Address, FromNativeType};

pub trait Data {
    fn size(&self) -> usize;

    fn copy(&self, offset: usize, target: &mut [u8]);

    fn load<R>(&self, offset: usize) -> R
    where
        R: FromNativeType,
    {
        let mut res = [0u8; 32];

        self.copy(offset, &mut res);

        builtin::mload(&res)
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
