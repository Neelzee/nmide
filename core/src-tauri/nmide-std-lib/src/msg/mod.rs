pub mod rmsg;
pub mod tmsg {
    use super::rmsg::RMsg;
    use crate::map::tmap::TValue;
    use serde::{Deserialize, Serialize};
    use std::mem::ManuallyDrop;
    use ts_rs::TS;

    #[derive(Serialize, Deserialize, TS, Clone)]
    #[ts(export)]
    pub enum TMsg {
        Msg(String, TValue),
    }

    impl From<RMsg> for TMsg {
        fn from(value: RMsg) -> Self {
            let tuple = unsafe { value.val.msg.clone() };
            Self::Msg(tuple.0.to_string(), tuple.1.to_owned().into())
        }
    }

    impl From<ManuallyDrop<RMsg>> for TMsg {
        fn from(value: ManuallyDrop<RMsg>) -> Self {
            let tuple = unsafe { value.val.msg.clone() };
            Self::Msg(tuple.0.to_string(), tuple.1.to_owned().into())
        }
    }
}
