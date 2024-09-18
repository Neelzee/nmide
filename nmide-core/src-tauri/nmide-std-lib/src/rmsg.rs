use crate::rmap::RValue;
use abi_stable::StableAbi;
use std::mem::ManuallyDrop;

#[repr(C)]
#[derive(StableAbi)]
pub struct RMsg {
    kind: RMsgKind,
    val: ManuallyDrop<RValue>,
}

#[repr(u8)]
#[derive(StableAbi)]
pub enum RMsgKind {
    Msg,
}
