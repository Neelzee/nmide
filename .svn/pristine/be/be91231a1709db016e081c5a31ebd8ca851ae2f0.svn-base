use std::{thread::sleep, time::Duration};

use core_std_lib::{core::Core, event::Event};

pub struct Module;

pub struct ModuleBuilder;

impl core_module_lib::ModuleBuilder for ModuleBuilder {
    fn build(self) -> impl core_module_lib::Module {
        Module
    }
}

#[async_trait::async_trait]
impl core_module_lib::Module for Module {
    fn name(&self) -> &str {
        "dummy_module"
    }

    async fn init(&self, _: Box<dyn Core>) {
        sleep(Duration::from_secs(1));
    }

    async fn handler(&self, _: Event, _: Box<dyn Core>) {}
}
