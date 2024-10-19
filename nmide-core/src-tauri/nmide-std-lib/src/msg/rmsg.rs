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

impl RMsg {
    pub fn new(kind: RMsgKind, val: RMsgUnion) -> Self {
        Self { kind, val }
    }

    pub fn kind(&self) -> &RMsgKind {
        &self.kind
    }

    pub fn val(&self) -> &RMsgUnion {
        &self.val
    }

    pub fn is_msg<S: ToString>(&self, key: S) -> bool {
        let s = key.to_string();
        unsafe { self.val.msg.0 == s }
    }
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

impl RMsgUnion {
    pub fn new(l: RString, r: RValue) -> Self {
        Self {
            msg: ManuallyDrop::new(Tuple2::from_tuple((l, r))),
        }
    }
}
