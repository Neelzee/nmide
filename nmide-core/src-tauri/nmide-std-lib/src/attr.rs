use crate::{css::Css, msg::Msg};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Attr {
    Id(String),
    Class(String),
    Alt(String),
    Src(String),
    For(String),
    Location(String),
    OnClick(Msg),
    Style(Vec<Css>),
    Unknown(String, String),
}

impl Attr {
    pub fn to_id(&self) -> Option<&str> {
        match self {
            Attr::Id(s) => Some(s),
            _ => None,
        }
    }
}
