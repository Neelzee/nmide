use anyhow::{Result, anyhow};
use core_std_lib::{
    attrs::Attr,
    core::Core,
    core_modification::CoreModification,
    event::Event,
    html::{Html, UIInstructionBuilder},
    state::Value,
};
use editor_ar::{Art, Position};

pub struct Module;

pub struct ModuleBuilder;

impl core_module_lib::ModuleBuilder for ModuleBuilder {
    fn build(self) -> impl core_module_lib::Module {
        Module
    }
}

const MODULE_NAME: &str = "editor_render";

const RENDER_BUFFER_EVENT: &str = "RENDER-BUFFER";

const BUFFER_STATE_FIELD: &str = "BUFFER";

#[async_trait::async_trait]
impl core_module_lib::Module for Module {
    fn name(&self) -> &str {
        MODULE_NAME
    }

    async fn init(&self, core: Box<dyn Core>) {
        core.add_handler(RENDER_BUFFER_EVENT.to_string(), MODULE_NAME.to_string())
            .await;
    }

    async fn handler(&self, event: Event, core: Box<dyn Core>) {
        let res: Result<()> = match &event {
            Event::Event { event, .. } if event == RENDER_BUFFER_EVENT => {
                render_buffer(&core).await
            }
            _ => Ok(()),
        };

        if res.is_ok() {
            return ();
        }

        let obj = Value::new_obj()
            .obj_add("error_event", Value::Str(event.event_name().to_string()))
            .obj_add("error_args", event.args().cloned().unwrap_or_default())
            .obj_add("error_msg", Value::Str(format!("{:?}", res.unwrap_err())));

        core.throw_event(Event::new("error", Some(obj))).await;
    }
}

async fn render_buffer(core: &Box<dyn Core>) -> Result<()> {
    let state = core.state().await;
    let art = state
        .get(BUFFER_STATE_FIELD)
        .and_then(Art::from_value)
        .ok_or(anyhow!(
            "Invalid value in field: {:?}",
            state.get(BUFFER_STATE_FIELD)
        ))?;

    core.send_modification(
        CoreModification::default()
            .set_ui(UIInstructionBuilder::default().add_node(render(art), Option::<String>::None)),
    )
    .await;
    Ok(())
}

fn render(art: Art) -> Html {
    match art {
        Art::Token { pos, char } => Html::Span()
            .set_text(char.to_string())
            .add_attr(pos_attr(pos)),
        Art::Group {
            pos,
            group,
            metadata,
        } => {
            let mut html = Html::Div().add_attr(pos_attr(pos));
            if let Some(class) = metadata.get("class").and_then(|v| v.str()) {
                html = html.add_attr(Attr::Class(class));
            }

            html.replace_kids(group.into_iter().map(render).collect())
        }
    }
}

fn pos_attr(pos: Position) -> Attr {
    Attr::Class(format!("pos-{:?}", pos.dec()))
}
