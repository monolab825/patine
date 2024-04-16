use patine_std::contract;

#[contract]
pub struct ExampleContract {}

#[contract]
impl ExampleContract {
    #[constructor]
    pub fn new() {}

    #[callable]
    pub fn test() {}

    #[callable]
    pub fn test() {}
}
