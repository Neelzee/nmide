use crate::map::rmap::RValue;
use abi_stable::{
    std_types::{RString, Tuple2},
    StableAbi,
};
use std::mem::ManuallyDrop;

#[repr(C)]
#[derive(StableAbi)]
pub struct RMsg {
    pub(crate) kind: RMsgKind,
    pub(crate) val: ManuallyDrop<RMsgUnion>,
}

impl Clone for RMsg {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind.clone(),
            val: ManuallyDrop::new(RMsgUnion {
                msg: unsafe { self.val.msg.clone() },
            }),
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
