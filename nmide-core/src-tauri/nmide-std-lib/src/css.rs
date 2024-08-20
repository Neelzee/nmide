use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Style {
    Width(f32, Unit),
    Height(f32, Unit),
    Padding(f32, Unit),
    PaddingTop(f32, Unit),
    PaddingBottom(f32, Unit),
    PaddingLeft(f32, Unit),
    PaddingRight(f32, Unit),
    Margin(f32, Unit),
    MarginTop(f32, Unit),
    MarginBottom(f32, Unit),
    MarginLeft(f32, Unit),
    MarginRight(f32, Unit),
    Color(Color),
    BackgroundColor(Color),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum StyleCondition {
    Class(String),
    Id(String),
    Kids,
    Siblings,
    Parent,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Css {
    pub styles: Vec<(StyleCondition, Style)>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Color {
    pub r: u8,
    pub b: u8,
    pub g: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Unit {
    Px,
    Em,
    Rem,
    Per,
    Ø,
}
