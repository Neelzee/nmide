//! The Nmide Foreign Standard Library
//!
//! Provides simple-ish interop with the Nmide-Core for Foreign Modules. For
//! more information regarding `Nmide-Core` please see the [`README`].
//!
//! This crate was developed to 1) modularize the core application, ensuring
//! separation of concerns and ease of unit-testing, and 2) to minimize the
//! dependencies for Foreign-Module-Developers.
//!
//! Using the [`abi_stable`] crate, Rust Modules can be dynamically and safely
//! loaded. Even if the Rust ABI is not guaranteed to be stable, [`abi_stable`]
//! can ensure this, and ensure no undefined behavior.
//!
//! The types used in the Core application are specified in the
//! [`Core Std. Lib.`].
//!
//! [`README`]: https://github.com/Neelzee/nmide
//! [`Core Std. Lib`]: https://github.com/Neelzee/nmide/blob/main/core/src-tauri/core-std-lib/src/lib.rs
//! [`ts-rs`]: https://docs.rs/ts-rs/latest/ts_rs/

/// Module containing *-Attr-Types
/// A mapping from the core std. lib. Attr to the relevant foreign module
pub mod attr;

/// Module containing *-Html-Types
/// A mapping from the core std. lib. Html to the relevant foreign module
pub mod html;

/// Module containing *-state & *-Value-Types
/// A mapping from the core std. lib. Value to the relevant foreign module
pub mod state;

/// Module containing *-events
/// A mapping from the core std. lib. Event to the relevant foreign module
pub mod event;

/// Module containing *-cores
/// A mapping from the core std. lib. Core to the relevant foreign module
pub mod core;

/// Module containing *-instr
/// A mapping from the core std. lib. Instruction to the relevant foreign module
pub mod instr;
