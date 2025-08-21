use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::state::tmap::TValue;

#[derive(Debug, TS, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct TEvent {
    event_name: String,
    module_name: String,
    args: Option<TValue>,
}
