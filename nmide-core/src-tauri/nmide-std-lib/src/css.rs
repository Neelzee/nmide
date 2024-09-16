use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Style {
    Width(String, Unit),
    Height(String, Unit),
    Padding(String, Unit),
    PaddingTop(String, Unit),
    PaddingLeft(String, Unit),
    PaddingBottom(String, Unit),
    PaddingRight(String, Unit),
    Margin(String, Unit),
    MarginTop(String, Unit),
    MarginBottom(String, Unit),
    MarginRight(String, Unit),
    MarginLeft(String, Unit),
    Color(Color),
    BackgroundColor(Color),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, TS)]
#[ts(export, export_to = "../../../app/bindings/TSStyle.ts")]
pub struct TSStyle {
    #[ts(optional = nullable)]
    width: Option<String>,
    #[ts(optional = nullable)]
    height: Option<String>,
    #[ts(optional = nullable)]
    padding: Option<String>,
    #[serde(rename = "paddingTop")]
    #[ts(optional = nullable)]
    padding_top: Option<String>,
    #[serde(rename = "paddingBottom")]
    #[ts(optional = nullable)]
    padding_bottom: Option<String>,
    #[serde(rename = "paddingRight")]
    #[ts(optional = nullable)]
    padding_right: Option<String>,
    #[serde(rename = "paddingLeft")]
    #[ts(optional = nullable)]
    padding_left: Option<String>,
    #[serde(rename = "marginTop")]
    #[ts(optional = nullable)]
    margin_top: Option<String>,
    #[serde(rename = "marginBottom")]
    #[ts(optional = nullable)]
    margin_bottom: Option<String>,
    #[serde(rename = "marginRight")]
    #[ts(optional = nullable)]
    margin_right: Option<String>,
    #[serde(rename = "marginLeft")]
    #[ts(optional = nullable)]
    margin_left: Option<String>,
    #[ts(optional = nullable)]
    color: Option<String>,
    #[serde(rename = "backgroundColor")]
    #[ts(optional = nullable)]
    background_color: Option<String>,
}

impl TSStyle {
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            padding: None,
            padding_top: None,
            padding_bottom: None,
            padding_right: None,
            padding_left: None,
            margin_top: None,
            margin_bottom: None,
            margin_right: None,
            margin_left: None,
            color: None,
            background_color: None,
        }
    }

    pub fn width<S>(self, s: S) -> Self
    where
        S: ToString,
    {
        Self {
            width: Some(s.to_string()),
            ..self
        }
    }

    pub fn padding_left<S>(self, s: S) -> Self
    where
        S: ToString,
    {
        Self {
            padding_left: Some(s.to_string()),
            ..self
        }
    }

    pub fn background_color<S>(self, s: S) -> Self
    where
        S: ToString,
    {
        Self {
            background_color: Some(s.to_string()),
            ..self
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color {
    pub r: u8,
    pub b: u8,
    pub g: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Unit {
    Px,
    Em,
    Rem,
    Per,
    Ø,
}
