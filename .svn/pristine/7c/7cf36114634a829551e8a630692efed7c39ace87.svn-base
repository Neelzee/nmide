use async_trait::async_trait;
use core_std_lib::{
    attrs::Attr,
    core::Core,
    core_modification::CoreModification,
    event::Event,
    html::{Html, UIInstructionBuilder},
    state::{StateInstructionBuilder, Value},
};
use module_installer::ModuleInstaller;
use std::fs;

pub(crate) mod module_installer;

pub struct ModuleBuilder;

impl core_module_lib::ModuleBuilder for ModuleBuilder {
    fn build(self) -> impl core_module_lib::Module {
        Module
    }
}

pub struct Module;

const NAME: &str = "module_installer";

#[async_trait]
impl core_module_lib::Module for Module {
    fn name(&self) -> &str {
        NAME
    }

    async fn init(&self, core: Box<dyn Core>) {
        core.add_handler("add-module".to_string(), NAME.to_string())
            .await;
        core.add_handler("toggle-add-module".to_string(), NAME.to_string())
            .await;
        let ui =
            Html::Div()
                .add_attr(Attr::Id("add-module-container".to_string()))
                .add_attr(Attr::Class("add-module-hide".to_string()))
                .adopt(
                    Html::Button()
                        .set_text("x")
                        .add_attr(Attr::Class("close-btn".to_string()))
                        .add_attr(Attr::Click(Event::new(
                            "toggle-add-module".to_string(),
                            None,
                        ))),
                )
                .adopt(
                    Html::Form()
                        .add_attr(Attr::Id("add-module-form".to_string()))
                        .adopt(
                            Html::Div()
                                .adopt(Html::Label().set_text("Module path: ").add_attr(
                                    Attr::Custom("for".to_string(), "module_path".to_string()),
                                ))
                                .adopt(Html::Input().add_attr(Attr::Custom(
                                    "name".to_string(),
                                    "module_path".to_string(),
                                ))),
                        )
                        .adopt(
                            Html::Div()
                                .adopt(Html::Label().set_text("GitHub url: ").add_attr(
                                    Attr::Custom("for".to_string(), "module_url".to_string()),
                                ))
                                .adopt(Html::Input().add_attr(Attr::Custom(
                                    "name".to_string(),
                                    "module_url".to_string(),
                                ))),
                        )
                        .adopt(
                            Html::Button()
                                .add_attr(Attr::Click(Event::new("add-module", None)))
                                .set_text("Install"),
                        ),
                );
        core.send_modification(CoreModification::ui(
            UIInstructionBuilder::default().add_node(ui, Option::<String>::None),
        ))
        .await;
    }

    async fn handler(&self, event: Event, core: Box<dyn Core>) {
        match (event.event_name(), event.args()) {
            ("toggle-add-module", _) => {
                let toggle = core
                    .state()
                    .await
                    .get("add-content-toggle")
                    .and_then(|v| v.bool())
                    .unwrap_or(true);
                core.send_modification(
                    CoreModification::ui(if toggle {
                        UIInstructionBuilder::default().add_attr(
                            "add-module-container".to_string(),
                            Attr::Class("add-module-show".to_string()),
                        )
                    } else {
                        UIInstructionBuilder::default().rem_attr(
                            Attr::Class("add-module-show".to_string()),
                            "add-module-container".to_string(),
                        )
                    })
                    .set_state(
                        StateInstructionBuilder::default()
                            .add("add-content-toggle", Value::Bool(!toggle)),
                    ),
                )
                .await;
            }
            ("add-module", Some(val)) => {
                let args = val.obj().unwrap().get("form").unwrap().obj().unwrap();
                let dir = core.appdir().await;
                if let Some(module_path) = args.get("module_path").unwrap().str() {
                    fs::copy(module_path, dir).unwrap();
                } else if let Some(url) = args.get("module_url").and_then(|v| v.str()) {
                    let installer = ModuleInstaller::new(dir.to_str().unwrap());
                    match installer.install_from_github(&url).await {
                        Ok(pth) => {
                            fs::copy(pth, dir).unwrap();
                        }
                        Err(err) => todo!("Handle error: ${err:?}"),
                    }
                }
            }
            _ => (),
        }
    }
}
