use abi_stable::library::{LibraryError, LibraryPath, RootModule};
use async_trait::async_trait;
use core_std_lib::{core::Core, core_modification::CoreModification, event::Event};
use foreign_std_lib::module::rs_module::ModuleRef;
use std::path::Path;

pub trait ModuleBuilder {
    fn build(self) -> impl Module;
}

#[async_trait]
pub trait Module: Send + Sync {
    fn name(&self) -> &str;
    async fn init(&self, core: Box<dyn Core>);
    async fn handler(&self, event: Event, core: Box<dyn Core>);
}

pub struct RsModule {
    module: ModuleRef,
    module_path: String,
}

impl RsModule {
    pub fn new(path: &Path) -> Result<Self, LibraryError> {
        Ok(Self {
            module: ModuleRef::load_from(LibraryPath::FullPath(path))?,
            module_path: path.to_string_lossy().to_string(),
        })
    }

    pub fn path(&self) -> &str {
        &self.module_path
    }
}
