#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

pub mod handlers;
#[cfg(feature = "ide")]
pub mod ide;
#[cfg(feature = "server")]
pub mod server;
pub mod setup;
pub mod statics;
