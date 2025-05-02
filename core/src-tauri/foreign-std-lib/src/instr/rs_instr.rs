use abi_stable::{std_types::{RBox, RString}, StableAbi};


#[repr(C)]
#[derive(StableAbi, Clone)]
pub enum RInstr<T> {
   NoOp,
   Add(RString, T),
   Rem(RString, T),
   Then(RBox<RInstr<T>>, RBox<RInstr<T>>),
}