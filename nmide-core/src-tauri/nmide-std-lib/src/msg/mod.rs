pub mod rmsg;
pub mod tmsg {
    use crate::map::tmap::TValue;
    use serde::{Deserialize, Serialize};
    use ts_rs::TS;

    #[derive(Serialize, Deserialize, TS)]
    #[ts(export)]
    pub enum TMsg {
        Msg(String, TValue),
    }
}
