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

Please add your contract file under `src`.
