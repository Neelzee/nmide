use async_trait::async_trait;
use core_module_lib::{self, Module};
use core_std_lib::attrs::Attr;
use core_std_lib::core::{Core, CoreModification};
use core_std_lib::event::Event;
use core_std_lib::html::{Html, UIInstructionBuilder};

pub struct ModuleBuilder;

impl core_module_lib::ModuleBuilder for ModuleBuilder {
    fn build(self) -> impl Module {
        FrameworkModule
    }
}

const MODULE_NAME: &str = "ide_framework";

struct FrameworkModule;

#[async_trait]
impl Module for FrameworkModule {
    fn name(&self) -> &str {
        MODULE_NAME
    }

    async fn init(&self, core: Box<dyn Core>) {
        let mods = CoreModification::default().set_ui(
            UIInstructionBuilder::default().add_node(
                Html::Main()
                    .add_attr(Attr::Id("root".to_string()))
                    .adopt(Html::Div().add_attr(Attr::Id("navbar".to_string())))
                    .adopt(
                        Html::Div()
                            .add_attr(Attr::Id("sidebar".to_string()))
                            .adopt(
                                Html::Div()
                                    .add_attr(Attr::Id("project".to_string()))
                                    .adopt(
                                        Html::Span()
                                            .set_text("Project")
                                            .add_attr(Attr::Id("project-title".to_string())),
                                    )
                                    .adopt(
                                        Html::Div().add_attr(Attr::Id("project-body".to_string())),
                                    ),
                            )
                            .adopt(Html::Ol().add_attr(Attr::Id("errors".to_string()))),
                    ),
                None,
                None,
            ),
        );
        core.get_sender()
            .await
            .send(mods)
            .await
            .expect("Channel should be opened");
    }

    async fn handler(&self, _: Event, _: Box<dyn Core>) {}
}
