pub mod rattr;
pub mod tattr {
    use crate::msg::tmsg::TMsg;
    use serde::{Deserialize, Serialize};
    use ts_rs::TS;

    use super::rattr::{RAttr, RAttrKind};

    #[derive(Serialize, Deserialize, TS)]
    #[ts(export)]
    pub enum TAttr {
        Id(String),
        Class(String),
        Style(String),
        OnClick(TMsg),
    }

    impl From<RAttr> for TAttr {
        fn from(value: RAttr) -> Self {
            match value.kind {
                RAttrKind::Id => Self::Id(value.str().unwrap().to_string()),
                RAttrKind::Class => Self::Class(value.str().unwrap().to_string()),
                RAttrKind::Style => Self::Style(value.str().unwrap().to_string()),
                RAttrKind::OnClick => Self::OnClick(value.msg().unwrap().clone().into()),
            }
        }
    }

    impl From<&RAttr> for TAttr {
        fn from(value: &RAttr) -> Self {
            match value.kind {
                RAttrKind::Id => Self::Id(value.str().unwrap().to_string()),
                RAttrKind::Class => Self::Class(value.str().unwrap().to_string()),
                RAttrKind::Style => Self::Style(value.str().unwrap().to_string()),
                RAttrKind::OnClick => Self::OnClick(value.msg().unwrap().clone().into()),
            }
        }
    }
}
