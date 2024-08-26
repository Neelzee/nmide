use nmide_std_lib::{
    attr::Attr,
    html::Html,
    map::{value::Value, Map},
};

use crate::PATH_KEY;

pub(crate) fn explorer(model: Map) -> Html {
    match model.lookup(PATH_KEY) {
        Some(Value::String(s)) if !s.is_empty() => Html::Div {
            kids: vec![Html::Text(format!("Error: {s}"))],
            attrs: Vec::new(),
        },
        Some(Value::List(files)) => Html::Div {
            kids: files.into_iter().map(|v| render_files(v)).collect(),
            attrs: Vec::new(),
        },
        _ => Html::Div {
            kids: vec![Html::Text("No Path Given".to_string())],
            attrs: Vec::new(),
        },
    }
}

fn render_files(v: Value) -> Html {
    match v {
        Value::String(s) => Html::P {
            kids: vec![Html::Text(s)],
            attrs: Vec::new(),
        },
        Value::List(mut xs) => Html::Div {
            kids: {
                let folder_name = xs.pop().unwrap().to_string().unwrap_or_default();
                xs.into_iter().fold(
                    vec![Html::P {
                        kids: vec![Html::Text(folder_name)],
                        attrs: Vec::new(),
                    }],
                    |mut acc, val| {
                        acc.push(render_files(val));
                        acc
                    },
                )
            },
            attrs: Vec::new(),
        },
        _ => Html::Frag {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
    }
}
