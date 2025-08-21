use async_trait::async_trait;
use core_std_lib::{core::Core, event::Event};

#[cfg(feature = "rs")]
pub mod rs_module;

pub trait ModuleBuilder {
    fn build(self) -> impl Module;
}

#[async_trait]
pub trait ModuleWrapper: Send + Sync {
    fn module_name(&self) -> String;
    async fn init(&self);
    async fn handler(&self, event: Event);
}

#[async_trait]
pub trait Module: Send + Sync {
    fn name(&self) -> &str;
    async fn init(&self, core: Box<dyn Core>);
    async fn handler(&self, event: Event, core: Box<dyn Core>);
}
