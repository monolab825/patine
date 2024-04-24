# Patine

> Write EVM contract using Rust.

## Features and TODO

- [ ] Core library.
  - [X] FFI
  - [X] Warpped Function
  - [X] UInt
  - [X] BytesN
  - [X] SInt
  - [X] Address
  - [ ] Add `AsRef` and `AsMut`
  - [X] Add From
  - [ ] Add `TryFrom`
  - [ ] Add "UnsafeFrom" and `UnsafeInto`
  - [X] Memory
- [ ] Std
  - [X] Selector
  - [ ] ABI
  - [ ] Contract
  - [ ] Context
    - [ ] Call
    - [ ] Transaction
    - [ ] Block
- [ ] Macros
  - [ ] Contract
  - [ ] uint!
  - [ ] Event
  - [ ] AbiEncode, AbiDecode
  - [ ] Function Call

## Perquisites

Install `cargo-evm` by `cargo binstall`.

```bash
cargo binstall cargo-evm
```

## Create Contract Project

Create project using the following command:

```bash
cargo evm new "<your-project-name>"
```

## Write Contract

Please add your contract file under `src`. We use simple store contract as example.

```rust
use patine::{storage::{Value, Storage}, U256};

#[contract]
pub struct Contract {
    value: Value<U256, Storage>,
}

#[contract]
impl Contract {
    #[expose]
    pub fn store(&mut self, value: U256) {
        self.value.set(value)
    }

    #[expose]
    pub fn load() -> U256 {
        self.value.get()
    }
}
```
