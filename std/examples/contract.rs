use patine_std::contract;

#[contract]
pub struct ExampleContract {
    store: Value<U256, Storage>,

    mapping: Map<U256, U256, Storage>,

    #[ctx]
    ctx: Context,
}

#[contract]
impl ExampleContract {
    #[constructor]
    pub fn new(&mut self) {}

    #[entry]
    pub fn entry(&mut self) {}
}
