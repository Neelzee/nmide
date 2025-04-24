use serde::{Deserialize, Serialize};
use ts_rs::TS;
use crate::event::Event;

// TODO: Correct documentation
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, TS)]
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
        match (self, s.to_lowercase().as_str()) {
            (Attr::Id(_), "id") => true,
            (Attr::Class(_), "class") => true,
            (Attr::Style(_), "style") => true,
            (Attr::Type(_), "type") => true,
            (Attr::Checked(_), "checked") => true,
            (Attr::Click(_), "onClick") => true,
            (Attr::OnInput(_), "onInput") => true,
            (Attr::EmitInput(_), "emitInput") => true,
            (Attr::Src(_), "src") => true,
            _ => false,
        }
    }
}
