//! *Attr
//!
//! An Attr type is a representation of an HTML-attribute. This mapping is not total, and will not
//! be total, but it does include some of the bare-essentials to create a simple Plugin.

/// Rust-Attr
pub mod rattr;
#[cfg(feature = "ts")]
/// TypeScript Attr
pub mod tattr {
    use crate::msg::tmsg::TMsg;
    use serde::{Deserialize, Serialize};
    use ts_rs::TS;

    use super::rattr::{RAttr, RAttrKind};

    #[derive(Serialize, Deserialize, TS)]
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
        /// Where `msg` is the value [`TMsg`] in `OnClick`
        ///
        /// [`TMsg`]: ../msg/mod.rs
        OnClick(TMsg),
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
        /// Where `msg` is the value [`TMsg`] in `OnInput`
        ///
        /// [`TMsg`]: ../msg/mod.rs
        OnInput(TMsg),
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

    impl From<RAttr> for TAttr {
        fn from(value: RAttr) -> Self {
            (&value).into()
        }
    }

    impl From<&RAttr> for TAttr {
        fn from(value: &RAttr) -> Self {
            match value.kind {
                RAttrKind::Id => Self::Id(value.str().unwrap_or_default().to_string()),
                RAttrKind::Class => Self::Class(value.str().unwrap_or_default().to_string()),
                RAttrKind::Style => Self::Style(value.str().unwrap_or_default().to_string()),
                RAttrKind::OnClick => Self::OnClick(value.msg().unwrap().clone().into()),
                RAttrKind::Src => Self::Src(value.str().unwrap_or_default().to_string()),
                RAttrKind::OnInput => Self::OnInput(value.msg().unwrap().clone().into()),
                RAttrKind::EmitInput => {
                    Self::EmitInput(value.str().unwrap_or_default().to_string())
                }
                RAttrKind::Type => Self::Type(value.str().unwrap_or_default().to_string()),
                RAttrKind::Checked => Self::Checked(value.bool().unwrap_or_default()),
            }
        }
    }
}
