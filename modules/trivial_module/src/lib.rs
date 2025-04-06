use core_std_lib::{
    attrs::Attr,
    core::{Core, CoreModification},
    event::Event,
    html::{Html, UIInstructionBuilder},
    state::{StateInstructionBuilder, Value},
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
        let state = StateInstructionBuilder::default().add("counter".to_string(), Value::Int(0));
        let ui = UIInstructionBuilder::default().add_node(
            Html::Div()
                .adopt(
                    Html::Button()
                        .text("Click")
                        .add_attr(Attr::OnClick(Event::new(
                            "counter".to_string(),
                            "trivial_module".to_string(),
                            Some(Value::Int(1)),
                        )))
                        .add_attr(Attr::Id("ButtonID".to_string())),
                )
                .add_attr(Attr::Id("DivId".to_string())),
            None,
            None,
        );
        CoreModification::default().set_ui(ui).set_state(state)
    }

    fn handler(&self, event: &Event, _core: &T) -> CoreModification {
        match (event.event_name(), event.args()) {
            ("counter", Some(v)) => {
                let state = StateInstructionBuilder::default().modify(
                    "counter".to_string(),
                    v.clone(),
                    |l, r| match (l, r) {
                        (Value::Int(i), Value::Int(j)) => Value::Int(i + j),
                        _ => Value::Int(0),
                    },
                );
                CoreModification::default().set_state(state)
            }
            _ => CoreModification::default(),
        }
    }
}
