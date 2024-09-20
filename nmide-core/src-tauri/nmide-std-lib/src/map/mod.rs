pub mod rmap;
pub mod tmap {
    use serde::{Deserialize, Serialize};
    use ts_rs::TS;

    #[derive(Serialize, Deserialize, TS)]
    #[ts(export)]
    pub enum TValue {
        Int(i32),
        Float(f32),
        Bool(bool),
        Str(String),
        List(Vec<TValue>),
        Obj(Vec<(String, TValue)>),
    }

    #[derive(Serialize, Deserialize, TS)]
    #[ts(export)]
    pub struct TMap {
        map: Vec<(String, TValue)>,
    }
}
