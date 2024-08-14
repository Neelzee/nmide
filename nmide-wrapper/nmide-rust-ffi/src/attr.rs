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
