use crate::{css::Css, msg::Msg};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Attr {
    Id(String),
    Class(String),
    Alt(String),
    Src(String),
    For(String),
    OnClick(Msg),
    Style(Vec<Css>),
    Attr(String, String),
}

impl Attr {
    pub fn to_id(&self) -> Option<&str> {
        match self {
            Attr::Id(s) => Some(s),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Attr::Id(_) => "id".to_string(),
            Attr::Class(_) => "class".to_string(),
            Attr::Alt(_) => "alt".to_string(),
            Attr::Src(_) => "src".to_string(),
            Attr::For(_) => "for".to_string(),
            Attr::OnClick(_) => "onclick".to_string(),
            Attr::Style(_) => "style".to_string(),
            Attr::Attr(a, _) => a.to_string(),
        }
    }
}
