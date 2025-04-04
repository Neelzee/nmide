use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS)]
//#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct TEvent;
