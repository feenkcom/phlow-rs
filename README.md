# phlow-rs
The engine for scripting reactive browsers in Rust. `Phlow` allows developers to add phlow views to any structure defined in other crates, including generic structures.

## Notice
This is a `nightly` crate because it relies on `specialization` (or `min_specialization`) features. See [https://github.com/rust-lang/rust/issues/31844](https://github.com/rust-lang/rust/issues/31844).

## Crates
For more information about each member crate consider checking out:
 - [phlow](./phlow) - core crate containing the engine
 - [phlow-derive](./phlow-derive) - derive crate used to create new extensions