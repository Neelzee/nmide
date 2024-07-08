pub mod either;
pub mod errors;
pub mod osops;
#[cfg(test)]
pub mod test;
pub mod types;
pub mod utils;
pub mod workspace;

use crate::nmide::workspace::Workspace;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

pub static WORKSPACE: Lazy<Mutex<Workspace>> = Lazy::new(|| Mutex::new(Workspace::empty()));
