use core_module_lib::Module;
use core_std_lib::core::Core;
use core_std_lib::event::Event;

pub(crate) struct EmptyModule;

#[async_trait::async_trait]
impl Module for EmptyModule {
    fn name(&self) -> &str {
        unimplemented!()
    }

    async fn init(&self, _: Box<dyn Core>) {
        unimplemented!()
    }

    async fn handler(&self, _: Event, _: Box<dyn Core>) {
        unimplemented!()
    }
}
