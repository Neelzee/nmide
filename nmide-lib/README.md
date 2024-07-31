# Nmide-LibC

C library for Nmide, to ensure safe FFI's

---

Nmide is made using Rust, and to ensure similar safety when using Plugins,
Nmide-LibC is made to bridge the gap between Rust, and other Programming
Lanugages that can has bindings to C. [nmide-rust-ffi](https://git.app.uib.no/Nils.Fitjar/nmide)
is a Rust Crate, that has a Rust wrapper around Nmide-LibC, so that *normal*
Rust types can be used.
