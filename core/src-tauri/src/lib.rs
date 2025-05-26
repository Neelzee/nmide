//! core_lib contains the necessary functions and types to create a zero-core
//! modular application.

/// Module for the `App`-trait, used by the `Core` to handle rerendering of the
/// UI, Event emitting, and exiting of the application.
pub mod app;

/// The different `Core` instances are implemented here, along with the
/// `ModuleEventRegister`, which is responsible for mapping Events and Modules.
pub mod core;
/// Handles core modifications on a separate thread
pub mod core_modification_handler;
/// Init and handler functions, for handling the initialization stage of
/// Modules, and possible Event handling stages.
pub mod handlers;
/// Contains Tauri specific code
pub mod ide;
pub mod setup;
pub mod statics;
