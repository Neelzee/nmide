use crate::instruction::inst::Instruction;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

impl<T: PartialEq + Clone + Eq + Hash + Debug> Instruction<T> {
    pub fn equivalent(&self, other: &Self) -> bool {
        let l = self.clone().flatten();
        let r = other.clone().flatten();
        Self::opt(&l) == Self::opt(&r)
    }

    pub fn is_noop(&self) -> bool {
        matches!(self, Self::NoOp)
    }

    pub fn flatten(self) -> Vec<Instruction<T>> {
        match &self {
            Instruction::NoOp | Instruction::Add(..) | Instruction::Rem(..) => vec![self],
            Instruction::Then(f, s) => {
                let mut xs = f.clone().flatten();
                xs.append(&mut s.clone().flatten());
                xs
            }
        }
    }

    pub fn unflatten(mut xs: Vec<Instruction<T>>) -> Self {
        if xs.len() == 1 {
            xs.pop().unwrap()
        } else {
            xs.into_iter()
                .fold(Self::NoOp, |acc, instr| acc.combine(instr))
        }
    }

    /// Creates an optimal instruction set by flattening the instructions, and removing all
    /// operations that are "NoOp"s
    pub fn optimize(vs: Vec<Instruction<T>>) -> Instruction<T> {
        let sequence: Vec<Instruction<T>> = Self::opt(&vs)
            .flatten()
            .into_iter()
            .filter(|i| !matches!(i, Instruction::NoOp))
            .collect();

        if sequence.is_empty() {
            return Instruction::NoOp;
        }

        let mut fv_map: HashMap<(String, T), i32> = HashMap::new();

        for instr in sequence.clone() {
            match instr {
                Instruction::Add(f, v) => {
                    let key = (f, v);
                    match fv_map.get(&key) {
                        Some(v) => {
                            fv_map.insert(key, *v + 1);
                        },
                        None => {
                            fv_map.insert(key, 1);
                        }
                    }
                }
                Instruction::Rem(f, v) => {
                    let key = (f, v);
                    match fv_map.get(&key) {
                        Some(v) => {
                            fv_map.insert(key, *v - 1);
                        },
                        None => {
                            fv_map.insert(key, -1);
                        }
                    }
                }
                _ =>
                    unreachable!("`Then` or `NoOp` instruction should never occur in a flattened instruction set"),
            }
        }

        sequence
            .into_iter()
            .fold(Instruction::NoOp, |acc, instr| match instr {
                Instruction::Add(f, v) => {
                    let key = (f.clone(), v.clone());
                    let i = *fv_map
                        .get(&key)
                        .expect("Should be initialized in the previous pass");
                    if i > 0 {
                        fv_map.insert(key, 0);
                        acc.combine(Instruction::Add(f, v))
                    } else {
                        acc
                    }
                }
                Instruction::Rem(f, v) => {
                    let key = (f.clone(), v.clone());
                    let i = *fv_map
                        .get(&key)
                        .expect("Should be initialized in the previous pass");
                    if i < 0 {
                        fv_map.insert(key, 0);
                        acc.combine(Instruction::Rem(f, v))
                    } else {
                        acc
                    }
                }
                Instruction::NoOp | Instruction::Then(..) => unreachable!(
                    "`NoOp` or `Then` instruction should never occur in a flattened instruction set"
                ),
            })
    }

    /// Removes all "NoOp"s from an instruction set
    ///
    /// If the instruction set is empty, or is a singleton with an `NoOp`
    /// instruction, it will return an `NoOp`
    pub fn opt(xs: &[Instruction<T>]) -> Instruction<T> {
        let ys: Vec<Instruction<T>> = xs
            .iter()
            .filter(|i| !matches!(i, Instruction::NoOp))
            .cloned()
            .collect();
        if ys.is_empty() {
            return Self::NoOp;
        }

        fn _opt<T: PartialEq + Clone + Eq + Hash + Debug>(ys: &[Instruction<T>]) -> Instruction<T> {
            match ys {
                [] => unreachable!("Inputted list in nested function is always non-empty"),
                [z] => z.clone(),
                [Instruction::NoOp, zs @ ..] => _opt(zs),
                [z, zs @ ..] => match z {
                    Instruction::NoOp => _opt(zs),
                    Instruction::Then(fst, snd) if matches!(*(fst).clone(), Instruction::NoOp) => {
                        _opt((*(snd.clone())).flatten().as_slice())
                    }
                    Instruction::Then(fst, snd) if matches!(*(snd).clone(), Instruction::NoOp) => {
                        _opt((*(fst.clone())).flatten().as_slice())
                    }
                    Instruction::Then(fst, snd) => Instruction::Then(
                        Box::new(_opt((*(fst.clone())).flatten().as_slice())),
                        Box::new(_opt((*(snd.clone())).flatten().as_slice())),
                    ),
                    _ => Instruction::Then(Box::new(z.clone()), Box::new(_opt(zs))),
                },
            }
        }
        _opt(ys.as_ref())
    }
}
