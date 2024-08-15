# Nmide-Rust-FFI

Wrapper Crate for Nmide, to interact with `Nmide-Lib`.

## How it works

Using [bindgen](https://github.com/rust-lang/rust-bindgen), a basic Rust-C-FFI is created.
However, this uses a lot of `unsafe`-types, -functions, which are unsafe, and
not as easy to work with. Therefore, this wrapper library can act as a safe translation
between the C way and Rust way.

It also creates TS-types for the frontend to use

To generate TS-types:

```shell
cargo test
```

To generate bindings:

```shell
bindgen ../../nmide-lib/nmide.h -o src/bindings.rs
```

cargo build --features test-lib --release
