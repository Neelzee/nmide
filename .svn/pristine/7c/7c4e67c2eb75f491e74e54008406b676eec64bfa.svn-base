//! *Map
//!
//! Contains a Json-like type, that allows sending of complicated structures between Plugins.

// TODO: Add doc-string

/// Rust-Map
#[cfg(feature = "rs")]
pub mod rs_state;
#[cfg(feature = "ts")]
/// TypeScript Map
pub mod tmap {
    use super::rs_state::{RKeyPair, RState, RValue};
    use serde::{Deserialize, Serialize};
    use ts_rs::TS;

    #[derive(Debug, Serialize, Deserialize, TS, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[ts(export_to = "TMap.ts")]
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
                super::rs_state::RValKind::Int => TValue::Int(value.int().unwrap()),
                super::rs_state::RValKind::Float => TValue::Float(value.float().unwrap()),
                super::rs_state::RValKind::Bool => TValue::Bool(value.bool().unwrap()),
                super::rs_state::RValKind::Str => {
                    TValue::Str(value.str().unwrap().clone().to_string())
                }
                super::rs_state::RValKind::List => TValue::List(
                    value
                        .lst()
                        .unwrap()
                        .iter()
                        .map(|v| v.clone().into())
                        .collect(),
                ),
                super::rs_state::RValKind::Obj => TValue::Obj(
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

    #[derive(Debug, Serialize, Deserialize, TS, Clone)]
    #[ts(export)]
    pub struct TMap(pub(crate) Vec<(String, TValue)>);

    impl TMap {
        pub fn new() -> Self {
            Self(Vec::new())
        }
    }

    impl Default for TMap {
        fn default() -> Self {
            Self::new()
        }
    }

    impl From<RState> for TMap {
        fn from(value: RState) -> Self {
            Self(value.pairs.iter().map(|v| v.clone().into()).collect())
        }
    }
}
mod impls;
