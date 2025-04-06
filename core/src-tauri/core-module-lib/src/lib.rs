use abi_stable::library::{LibraryError, LibraryPath, RootModule};
use core_std_lib::{
    core::{Core, CoreModification},
    event::Event,
};
use foreign_std_lib::module::rs_module::ModuleRef;
use std::{path::Path, sync::Arc};

pub mod js_module;

pub trait ModuleBuilder {
    fn build<T: Core>(self) -> Module<T>;
}

pub trait ModuleTrait<T>: Send + Sync {
    fn name(&self) -> &str;
    fn init(&self, core: &T) -> CoreModification;
    fn handler(&self, event: &Event, core: &T) -> CoreModification;
}

pub struct Module<T>
where
    T: Core,
{
    handle: Arc<dyn ModuleTrait<T>>,
}

impl<T> Module<T>
where
    T: Core,
{
    pub fn new(module: impl ModuleTrait<T> + 'static) -> Self {
        Self {
            handle: Arc::new(module),
        }
    }

    pub fn name(&self) -> &str {
        self.handle.name()
    }

    pub fn init(&self, core: &T) -> CoreModification {
        self.handle.init(core)
    }

    pub fn handler(&self, event: &Event, core: &T) -> CoreModification {
        self.handle.handler(event, core)
    }
}

pub struct RsModule {
    module: ModuleRef,
    module_path: String,
}

impl<T> ModuleTrait<T> for RsModule {
    fn name(&self) -> &str {
        self.module_path.split("/").last().unwrap_or_default()
    }

    fn init(&self, core: &T) -> CoreModification {
        self.module.init()(0);
        todo!()
    }

    fn handler(&self, event: &Event, core: &T) -> CoreModification {
        self.module.handler()(0);
        todo!()
    }
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
