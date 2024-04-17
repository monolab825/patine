use patine_std::{contract, Address, Contract};

#[derive(Default)]
pub struct ExampleContract {
    __address: Option<Address>,
}

impl Contract for ExampleContract {
    fn new(address: Address) -> Self {
        Self {
            __address: Some(address),
        }
    }

    fn selfaddress(&self) -> Option<&Address> {
        self.__address.as_ref()
    }
}

impl ExampleContract {
    fn constructor(&mut self) {
        let address = self.address();
    }

    fn entry(&mut self) {}
}

#[no_mangle]
pub extern "C" fn __ExampleContract_constructor() {
    let mut contract = ExampleContract::default();

    contract.constructor();
}
