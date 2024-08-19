use crate::css::Css;


#[derive(Debug, Clone)]
pub enum Attr {
    Id(String),
    Class(String),
    Alt(String),
    Src(String),
    For(String),
    Location(String),
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
