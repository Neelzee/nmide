pub mod rmap;
pub mod tmap {
    use super::rmap::{RKeyPair, RMap, RValue};
    use serde::{Deserialize, Serialize};
    use ts_rs::TS;

    #[derive(Serialize, Deserialize, TS, Clone)]
    #[ts(export)]
    pub enum TValue {
        Int(i32),
        Float(f32),
        Bool(bool),
        Str(String),
        List(Vec<TValue>),
        Obj(Vec<(String, TValue)>),
    }

    impl From<RValue> for TValue {
        fn from(value: RValue) -> Self {
            match value.kind {
                super::rmap::RValKind::Int => TValue::Int(value.int().unwrap()),
                super::rmap::RValKind::Float => TValue::Float(value.float().unwrap()),
                super::rmap::RValKind::Bool => TValue::Bool(value.bool().unwrap()),
                super::rmap::RValKind::Str => TValue::Str(value.str().unwrap().clone().to_string()),
                super::rmap::RValKind::List => TValue::List(
                    value
                        .lst()
                        .unwrap()
                        .iter()
                        .map(|v| v.clone().into())
                        .collect(),
                ),
                super::rmap::RValKind::Obj => TValue::Obj(
                    value
                        .obj()
                        .unwrap()
                        .iter()
                        .map(|v| v.clone().into())
                        .collect(),
                ),
            }
        }
    }

    impl From<RKeyPair> for (String, TValue) {
        fn from(value: RKeyPair) -> Self {
            (value.key.clone().to_string(), value.val.into())
        }
    }

    #[derive(Serialize, Deserialize, TS, Clone)]
    #[ts(export)]
    pub struct TMap {
        map: Vec<(String, TValue)>,
    }

    impl TMap {
        pub fn new() -> Self {
            Self { map: Vec::new() }
        }

        pub fn merge(self, other: Self) -> Self {
            unimplemented!()
        }
    }

    impl From<RMap> for TMap {
        fn from(value: RMap) -> Self {
            Self {
                map: value.pairs.iter().map(|v| v.clone().into()).collect(),
            }
        }
    }
}
