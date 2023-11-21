# phlow
The engine for scripting reactive browsers in Rust. `Phlow` allows developers to add phlow views to any structure defined in other crates, including generic structures.

## Notice
This is a `nightly` crate because it relies on `specialization` (or `min_specialization`) features. See [https://github.com/rust-lang/rust/issues/31844](https://github.com/rust-lang/rust/issues/31844).

## Depend

```toml
phlow = { version = "*" }
```

## Features
- `printing` - **enabled by default**, detects if an arbitrary type implements `Display` or `Debug` and uses an appropriate one in `PhlowObject::to_string`. As a result any object or reference wrapped in `phlow!()` becomes printable.
- `phlow-derive` - - **enabled by default**, enable to define new extensions, is not required to import existing extensions