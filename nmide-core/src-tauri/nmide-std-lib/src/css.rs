use crate::html::Element;

#[derive(Debug, Clone)]
pub enum Style {
    Width(f32),
    Height(f32),
    Padding(f32),
    PaddingTop(f32),
    PaddingBottom(f32),
    PaddingLeft(f32),
    PaddingRight(f32),
    Margin(f32),
    MarginTop(f32),
    MarginBottom(f32),
    MarginLeft(f32),
    MarginRight(f32),
    Color(Color),
    BackgroundColor(Color),
}

#[derive(Debug, Clone)]
pub enum StyleCondition {
    Class(String),
    Id(String),
    Kids,
    Siblings,
    Parent,
    Element(Element),
}

#[derive(Debug, Clone)]
pub struct Css {
    pub styles: Vec<(StyleCondition, Style)>,
}

#[derive(Debug, Clone)]
pub struct Color {
    pub r: u8,
    pub b: u8,
    pub g: u8,
}

#[derive(Debug, Clone)]
pub enum Unit {
    Pixel,
    Rem,
    Percentage,
}
