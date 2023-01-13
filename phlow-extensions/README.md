# phlow-extensions

Contains `phlow` extensions to `core` Rust types.

## Depend
```toml
phlow-extensions = "*"
```

## Import
```rust
/// To import core extensions from this crate add
/// the following to your `lib.rs` or `main.rs`:
#[macro_use]
extern crate phlow;

import_extensions!(CoreExtensions, ...);
```