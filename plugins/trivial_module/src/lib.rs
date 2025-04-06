use core_std_lib::{
    core::{Core, CoreModification},
    event::Event,
};

pub struct ModuleBuilder;

impl core_module_lib::ModuleBuilder for ModuleBuilder {
    fn build<T: Core>(self) -> core_module_lib::Module<T> {
        core_module_lib::Module::<T>::new(Module)
    }
}

pub struct Module;

impl<T> core_module_lib::ModuleTrait<T> for Module
where
    T: Core,
{
    fn name(&self) -> &str {
        "trivial module"
    }

    fn init(&self, _core: &T) -> CoreModification {
        CoreModification::default()
    }

    fn handler(&self, _event: &Event, _core: &T) -> CoreModification {
        CoreModification::default()
    }
}
