use crate::event::ts_event::TEvent;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum TAttr {
    /// ```html
    /// <div id="foobar"/>
    /// ```
    Id(String),
    /// ```html
    /// <div class="foobar"/>
    /// ```
    Class(String),
    Style(String),
    /// Only valid for `Input`
    /// ```html
    /// <input type="checkbox"/>
    /// ```
    Type(String),
    /// Only valid for `Input`
    /// ```html
    /// <input type="checkbox" checked="true"/>
    /// ```
    Checked(bool),
    /// Only valid for `Input`
    /// Given this DOM
    /// ```html
    /// <div id="foobar"/>
    /// ```
    /// The `OnClick`-Attributes has this effect on an HTML-Element:
    /// ```javascript
    /// const el = document.getElementById("foobar");
    /// el.addEventListener("click", () => {
    ///   window.emit("msg", msg);
    /// });
    /// ```
    /// Where `msg` is the value [`TEvent`] in `OnClick`
    ///
    /// [`TEvent`]: ../msg/mod.rs
    OnClick(TEvent),
    /// Only valid for `Input`
    /// Given this DOM
    /// ```html
    /// <div id="foobar"/>
    /// ```
    /// The `OnInput`-Attributes has this effect on an HTML-Element:
    /// ```javascript
    /// const el = document.getElementById("foobar");
    /// el.addEventListener("input", () => {
    ///   window.emit("msg", msg);
    /// });
    /// ```
    /// Where `msg` is the value [`TEvent`] in `OnInput`
    ///
    /// [`TEvent`]: ../msg/mod.rs
    OnInput(TEvent),
    /// Only valid for `Input`
    /// Given this DOM
    /// ```html
    /// <div id="foobar"/>
    /// ```
    /// The `OnClick`-Attributes has this effect on an HTML-Element:
    /// ```javascript
    /// const el = document.getElementById("foobar");
    /// el.addEventListener("input", () => {
    ///   window.emit("msg", { msg: [msgName, el.value] });
    /// });
    /// ```
    /// Where `msgName` is the value in `EmitInput`, and the name of the [`Msg`] being sent.
    ///
    /// [`Msg`]: ../msg/mod.rs
    EmitInput(String),
    /// Only valid for `Img`, `Video`, `Audio`, and `Script`
    /// ```html
    /// <img src="foobar"/>
    /// ```
    Src(String),
}
