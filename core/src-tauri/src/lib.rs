//! core_lib contains the necessary functions and types to create a zero-core
//! modular application.

pub mod apps;

/// Module for the `App`-trait, used by the `Core` to handle rerendering of the
/// UI, Event emitting, and exiting of the application.
pub mod platform;

/// The different `Core` instances are implemented here, along with the
/// `ModuleEventRegister`, which is responsible for mapping Events and Modules.
pub mod context;
pub mod core;

#[cfg(feature = "module-installer")]
pub mod installer;
