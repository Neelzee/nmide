use abi_stable::{
    StableAbi,
    std_types::{RBox, RString},
};
use core_std_lib::instruction::inst::Instruction;

#[repr(C)]
#[derive(StableAbi, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RInstr<T> {
    #[default]
    NoOp,
    Add(RString, T),
    Rem(RString, T),
    Then(RBox<RInstr<T>>, RBox<RInstr<T>>),
}

impl<T> From<Instruction<T>> for RInstr<T> {
    fn from(value: Instruction<T>) -> Self {
        match value {
            Instruction::NoOp => Self::NoOp,
            Instruction::Add(s, v) => Self::Add(RString::from(s), v),
            Instruction::Rem(s, v) => Self::Rem(RString::from(s), v),
            Instruction::Then(fst, snd) => {
                Self::Then(RBox::new(Self::from(*fst)), RBox::new(Self::from(*snd)))
            }
        }
    }
}

impl<T: Clone + PartialEq> RInstr<T> {
    pub fn flatten(&self) -> Vec<RInstr<T>> {
        match &self {
            RInstr::NoOp | RInstr::Add(_, _) | RInstr::Rem(_, _) => vec![self.clone()],
            RInstr::Then(rbox, rbox1) => {
                let mut xs = rbox.flatten();
                let mut ys = rbox1.flatten();
                xs.append(&mut ys);

                xs
            }
        }
    }

    pub fn unflatten(mut xs: Vec<Self>) -> Self {
        if xs.len() == 1 {
            xs.pop().unwrap()
        } else {
            xs.into_iter()
                .fold(Self::NoOp, |acc, instr| acc.combine(instr))
        }
    }

    pub fn equivalent(&self, other: &Self) -> bool {
        let l = self.clone().flatten();
        let r = other.clone().flatten();
        Self::opt(&l) == Self::opt(&r)
    }

    pub fn opt(xs: &[Self]) -> Self {
        let ys: Vec<Self> = xs
            .iter()
            .filter(|i| !matches!(i, Self::NoOp))
            .cloned()
            .collect();
        if ys.is_empty() {
            return Self::NoOp;
        }

        fn _opt<T: Clone + PartialEq>(ys: &[RInstr<T>]) -> RInstr<T> {
            match ys {
                [] => unreachable!("Inputted list in nested function is always non-empty"),
                [z] => z.clone(),
                [RInstr::NoOp, zs @ ..] => _opt(zs),
                [z, zs @ ..] => match z {
                    RInstr::NoOp => _opt(zs),
                    RInstr::Then(fst, snd) if matches!(*(fst).clone(), RInstr::NoOp) => {
                        _opt((*(snd.clone())).flatten().as_slice())
                    }
                    RInstr::Then(fst, snd) if matches!(*(snd).clone(), RInstr::NoOp) => {
                        _opt((*(fst.clone())).flatten().as_slice())
                    }
                    RInstr::Then(fst, snd) => RInstr::Then(
                        RBox::new(_opt((*(fst.clone())).flatten().as_slice())),
                        RBox::new(_opt((*(snd.clone())).flatten().as_slice())),
                    ),
                    _ => RInstr::Then(RBox::new(z.clone()), RBox::new(_opt(zs))),
                },
            }
        }
        _opt(ys.as_ref())
    }
}

impl<T: Clone> RInstr<T> {
    pub fn combine(self, other: Self) -> Self {
        match (&self, &other) {
            (RInstr::NoOp, _) => other,
            (_, RInstr::NoOp) => self,
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
