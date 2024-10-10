use crate::{map::rmap::RValue, msg::tmsg::TMsg};
use abi_stable::{
    std_types::{RString, Tuple2},
    StableAbi,
};
use std::{mem::ManuallyDrop, str::FromStr};

#[repr(C)]
#[derive(StableAbi)]
pub struct RMsg {
    pub(crate) kind: RMsgKind,
    pub(crate) val: RMsgUnion,
}

impl Clone for RMsg {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind.clone(),
            val: RMsgUnion {
                msg: unsafe { self.val.msg.clone() },
            },
        }
    }
}

impl From<TMsg> for RMsg {
    fn from(value: TMsg) -> Self {
        match value {
            TMsg::Msg(msg, val) => Self {
                kind: RMsgKind::Msg,
                val: RMsgUnion {
                    msg: ManuallyDrop::new(Tuple2::from_tuple((
                        RString::from_str(&msg).unwrap_or_default(),
                        val.into(),
                    ))),
                },
            },
        }
    }
}

#[repr(u8)]
#[derive(StableAbi, Clone)]
pub enum RMsgKind {
    Msg,
}

#[repr(C)]
#[derive(StableAbi)]
pub union RMsgUnion {
    pub(crate) msg: ManuallyDrop<Tuple2<RString, RValue>>,
}
