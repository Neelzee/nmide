#![warn(clippy::all, clippy::pedantic)]

/// Init, Update and View functions
pub mod handlers;
#[cfg(feature = "ide")]
pub mod ide;
#[cfg(feature = "server")]
pub mod server;
pub mod setup;
pub mod statics;
