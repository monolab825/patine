# Patine

> Write EVM contract using Rust.

## Features and TODO

- [ ] Core library.
  - [X] FFI
  - [ ] Warpped Function
  - [ ] UInt
  - [ ] BytesN
  - [ ] SInt
  - [ ] Address
- [ ] Std
  - [X] Memory
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

Please add your contract file under `src`.
