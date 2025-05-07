use abi_stable::{
    StableAbi,
    std_types::{RBox, RString},
};
use core_std_lib::instruction::inst::Instruction;

#[repr(C)]
#[derive(StableAbi, Clone, Default)]
pub enum RInstr<T> {
    #[default]
    NoOp,
    Add(RString, T),
    Rem(RString, T),
    Then(RBox<RInstr<T>>, RBox<RInstr<T>>),
}

impl<T: Clone> RInstr<T> {
    pub fn combine(self, other: Self) -> Self {
        match (&self, &other) {
            (RInstr::NoOp, _) => other,
            (_, RInstr::NoOp) => other,
            _ => Self::Then(RBox::new(self), RBox::new(other)),
        }
    }

    pub fn to_instr(&self) -> Instruction<T> {
        match self {
            RInstr::NoOp => Instruction::NoOp,
            RInstr::Add(s, v) => Instruction::Add(s.as_str().to_string(), v.clone()),
            RInstr::Rem(s, v) => Instruction::Rem(s.as_str().to_string(), v.clone()),
            RInstr::Then(fst, snd) => {
                Instruction::Then(Box::new(fst.to_instr()), Box::new(snd.to_instr()))
            }
        }
    }

    pub fn map<K, F>(&self, f: F) -> RInstr<K>
    where
        K: Clone,
        F: Fn(T) -> K + Clone,
    {
        match self {
            Self::NoOp => RInstr::<K>::NoOp,
            Self::Add(s, v) => RInstr::<K>::Add(s.clone(), f(v.clone())),
            Self::Rem(s, v) => RInstr::<K>::Rem(s.clone(), f(v.clone())),
            Self::Then(fst, snd) => {
                RInstr::<K>::Then(RBox::new(fst.map(f.clone())), RBox::new(snd.map(f)))
            }
        }
    }
}
