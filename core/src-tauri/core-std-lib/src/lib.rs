//! The Nmide Standard Library
//!
//! Provides simple-ish interop with the Nmide-Core for Rust Plugins. For more information
//! regarding `Nmide-Core` please see the [`README`].
//!
//! This crate was developed to 1) modularize the core application, ensuring separation of concerns
//! and ease of unit-testing, and 2) to minimize the dependencies for Rust-Plugin-Developers.
//!
//! Using the [`abi_stable`] crate, Rust Plugins can be dynamically and safely loaded. Even if the
//! Rust ABI is not guaranteed to be stable, [`abi_stable`] can ensure this, and ensure no
//! undefined behavior.
//!
//! The types used in the Core application are specified here. Both in the frontend, (T-Type), and
//! backend (R-Type). Using [`ts-rs`], The T-Types are transpiled to TypeScript Types, ensuring a
//! close to one-to-one relation between the R-Types and T-Types, the few differences being due to
//! the borrow-checker.
//!
//! [`README`]: https://github.com/Neelzee/nmide
//! [`ts-rs`]: https://docs.rs/ts-rs/latest/ts_rs/

/// Module containing the Attr struct
pub mod attrs;

/// Module containing *-Html-Types
pub mod html;

/// Module containing State
pub mod state;

pub mod core;

pub mod event;

pub mod instruction;
