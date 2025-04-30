use crate::event::Event;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

// TODO: Correct documentation
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, TS, Hash, Eq)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum Attr {
    /// ```html
    /// <div id="foobar"/>
    /// ```
    Id(String),
    /// ```html
    /// <div class="foobar"/>
    /// ```
    #[serde(rename = "clss")]
    Class(String),
    Style(String),
    /// only valid for `input`
    /// ```html
    /// <input type="checkbox"/>
    /// ```
    Type(String),
    /// only valid for `input`
    /// ```html
    /// <input type="checkbox" checked="true"/>
    /// ```
    Checked(bool),
    /// only valid for `input`
    /// given this dom
    /// ```html
    /// <div id="foobar"/>
    /// ```
    /// the `onclick`-attributes has this effect on an html-element:
    /// ```javascript
    /// const el = document.getelementbyid("foobar");
    /// el.addeventlistener("click", () => {
    ///   window.emit("msg", msg);
    /// });
    /// ```
    /// where `msg` is the value [`tmsg`] in `onclick`
    ///
    /// [`tmsg`]: ../msg/mod.rs
    Click(Event),
    /// only valid for `input`
    /// given this dom
    /// ```html
    /// <div id="foobar"/>
    /// ```
    /// the `oninput`-attributes has this effect on an html-element:
    /// ```javascript
    /// const el = document.getelementbyid("foobar");
    /// el.addeventlistener("input", () => {
    ///   window.emit("msg", msg);
    /// });
    /// ```
    /// where `msg` is the value [`tmsg`] in `oninput`
    ///
    /// [`tmsg`]: ../msg/mod.rs
    OnInput(Event),
    /// only valid for `input`
    /// given this dom
    /// ```html
    /// <div id="foobar"/>
    /// ```
    /// the `onclick`-attributes has this effect on an html-element:
    /// ```javascript
    /// const el = document.getelementbyid("foobar");
    /// el.addeventlistener("input", () => {
    ///   window.emit("msg", { msg: [msgname, el.value] });
    /// });
    /// ```
    /// where `msgname` is the value in `emitinput`, and the name of the [`msg`] being sent.
    ///
    /// [`msg`]: ../msg/mod.rs
    EmitInput(Event),
    /// only valid for `img`, `video`, `audio`, and `script`
    /// ```html
    /// <img src="foobar"/>
    /// ```
    Src(String),
}

impl Attr {
    pub fn as_string_rep(&self) -> &str {
        match self {
            Attr::Id(_) => "id",
            Attr::Class(_) => "class",
            Attr::Style(_) => "style",
            Attr::Type(_) => "type",
            Attr::Checked(_) => "checked",
            Attr::Click(_) => "onClick",
            Attr::OnInput(_) => "onInput",
            Attr::EmitInput(_) => "emitInput",
            Attr::Src(_) => "src",
        }
    }

    pub fn is(&self, s: &str) -> bool {
        matches!(
            (self, s.to_lowercase().as_str()),
            (Attr::Id(_), "id")
                | (Attr::Class(_), "class")
                | (Attr::Style(_), "style")
                | (Attr::Type(_), "type")
                | (Attr::Checked(_), "checked")
                | (Attr::Click(_), "onClick")
                | (Attr::OnInput(_), "onInput")
                | (Attr::EmitInput(_), "emitInput")
                | (Attr::Src(_), "src")
        )
    }

    pub fn has(&self, s: &str) -> bool {
        match self {
            Attr::Id(o) | Attr::Class(o) => o.split_ascii_whitespace().any(|a| a == s),
            _ => unimplemented!(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Attr::Id(o) | Attr::Class(o) => o.is_empty(),
            _ => unimplemented!(),
        }
    }

    pub fn value(&self) -> &str {
        match self {
            Attr::Id(o) | Attr::Class(o) => o,
            _ => unimplemented!(),
        }
    }

    pub fn remove(self, s: &str) -> Self {
        match self {
            Attr::Id(o) => Self::Id(
                o.split_ascii_whitespace()
                    .into_iter()
                    .filter(|a| a != &s)
                    .collect(),
            ),
            Attr::Class(o) => Self::Class(
                o.split_ascii_whitespace()
                    .into_iter()
                    .filter(|a| a != &s)
                    .collect(),
            ),
            _ => unimplemented!(),
        }
    }
}
