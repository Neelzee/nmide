use nmide_std_lib::{
    attr::Attr,
    html::Html,
    map::{value::Value, Map},
    msg::Msg,
};

use crate::{CLOSE_FILE_MSG, FILE_CONTENT_KEY};

pub(crate) fn editor(model: &Map) -> Html {
    let not_open = model
        .lookup(CLOSE_FILE_MSG)
        .and_then(|v| v.to_bool())
        .unwrap_or(false);
    match model.lookup(FILE_CONTENT_KEY) {
        Some(_) if !not_open => editor_open(model),
        _ => Html::Div(),
    }
}

fn editor_open(model: &Map) -> Html {
    Html::Div {
        kids: vec![
            Html::Button {
                kids: vec![Html::Text("Close".to_string())],
                attrs: vec![Attr::OnClick(Msg::PluginMsg(
                    CLOSE_FILE_MSG.to_string(),
                    Value::Int(0),
                ))],
            },
            Html::P {
                kids: vec![Html::Text(
                    model
                        .lookup(FILE_CONTENT_KEY)
                        .and_then(|v| v.to_string())
                        .unwrap_or_default(),
                )],
                attrs: Vec::new(),
            },
        ],
        attrs: Vec::new(),
    }
}
