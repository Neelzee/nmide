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

#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
use abi_stable::{
    library::{LibraryError, RootModule},
    package_version_strings,
    sabi_types::VersionStrings,
    StableAbi,
};
use html::rhtml::RHtml;
use map::rmap::RMap;
use msg::rmsg::RMsg;
use std::path::Path;

/// Module containing *-Attr-Types
pub mod attr;

/// Module containing *-Html-Types
pub mod html;

/// Module containing *-Map & *-Value-Types
pub mod map;

/// Module containing *-Msg-Types
pub mod msg;

pub mod core;

pub mod event;

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = NmideStandardLibraryRef)))]
#[sabi(missing_field(panic))]
pub struct NmideStdLib {
    pub init: extern "C" fn() -> RMap,
    pub view: extern "C" fn(model: &RMap) -> RHtml,
    pub update: extern "C" fn(msg: &RMsg, model: &RMap) -> RMap,
}

impl RootModule for NmideStandardLibraryRef {
    abi_stable::declare_root_module_statics! {NmideStandardLibraryRef}

    const BASE_NAME: &'static str = "nmide_example_plugin";
    const NAME: &'static str = "nmide_example_plugin";
    const VERSION_STRINGS: VersionStrings = package_version_strings!();
}

pub fn load_root_module_in_directory(
    directory: &Path,
) -> Result<NmideStandardLibraryRef, LibraryError> {
    NmideStandardLibraryRef::load_from_directory(directory)
}

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = NmideStandardModuleRef)))]
#[sabi(missing_field(panic))]
pub struct NmideModule {
    pub init: extern "C" fn(core: &Core) -> CoreModification,
    pub handle: extern "C" fn(event: &REvent, core: &Core) -> CoreModification,
}

impl RootModule for NmideStandardModuleRef {
    abi_stable::declare_root_module_statics! {NmideStandardModuleRef}

    const BASE_NAME: &'static str = "nmide_example_module";
    const NAME: &'static str = "nmide_example_module";
    const VERSION_STRINGS: VersionStrings = package_version_strings!();
}

pub fn load_root_module_in_directory(
    directory: &Path,
) -> Result<NmideStandardModuleRef, LibraryError> {
    NmideStandardModuleRef::load_from_directory(directory)
}
