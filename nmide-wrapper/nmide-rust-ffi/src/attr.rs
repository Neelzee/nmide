use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::model::Msg;

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub enum Attr {
    Id(String),
    Class(String),
    Src(String),
    Alt(String),
    OnClick(Msg),
    Attr(String, String),
}

impl Attr {
    pub fn to_string_id(&self) -> &str {
        match self {
            Attr::Id(_) => "id",
            Attr::Class(_) => "class",
            Attr::Src(_) => "src",
            Attr::Alt(_) => "alt",
            Attr::OnClick(_) => "onclick",
            Attr::Attr(a, _) => &a,
        }
    }
}
