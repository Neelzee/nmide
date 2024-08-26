use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, TS)]
#[ts(export, export_to = "../../../src/bindings/Style.ts")]
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
#[ts(export, export_to = "../../../src/bindings/StyleCondition.ts")]
pub enum StyleCondition {
    Class(String),
    Id(String),
    Kids,
    Siblings,
    Parent,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, TS)]
#[ts(export, export_to = "../../../src/bindings/Css.ts")]
pub struct Css {
    pub styles: Vec<(StyleCondition, Style)>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, TS)]
#[ts(export, export_to = "../../../src/bindings/Color.ts")]
pub struct Color {
    pub r: u8,
    pub b: u8,
    pub g: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, TS)]
#[ts(export, export_to = "../../../src/bindings/Unit.ts")]
pub enum Unit {
    Px,
    Em,
    Rem,
    Per,
    Ø,
}
