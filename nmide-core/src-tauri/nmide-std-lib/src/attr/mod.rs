pub mod rattr;
pub mod tattr {
    use crate::msg::tmsg::TMsg;
    use serde::{Deserialize, Serialize};
    use ts_rs::TS;

    #[derive(Serialize, Deserialize, TS)]
    #[ts(export)]
    pub enum TAttr {
        Id(String),
        Class(String),
        Style(String),
        OnClick(TMsg),
    }
}
