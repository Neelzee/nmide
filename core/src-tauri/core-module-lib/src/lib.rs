use abi_stable::library::{LibraryError, LibraryPath, RootModule};
use async_trait::async_trait;
use core_std_lib::{core::Core, core_modification::CoreModification, event::Event};
use std::path::Path;

#[cfg(feature = "rs")]
pub mod rs_module;

pub trait ModuleBuilder {
    fn build(self) -> impl Module;
}

#[async_trait]
pub trait Module: Send + Sync {
    fn name(&self) -> &str;
    async fn init(&self, core: Box<dyn Core>);
    async fn handler(&self, event: Event, core: Box<dyn Core>);
}