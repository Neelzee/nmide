use std::collections::HashMap;
use crate::instruction::inst::Instruction;

impl<T: PartialEq + Clone> Instruction<T> {
    pub fn is_noop(&self) -> bool {
        matches!(self, Self::NoOp)
    }

    fn flatten(self) -> Vec<Instruction<T>> {
        match &self {
            Instruction::NoOp => Vec::new(),
            Instruction::Add(..) | Instruction::Rem(..) => vec![self],
            Instruction::Then(f, s) => {
                let mut xs = f.flatten();
                xs.append(&mut s.flatten());
                xs
            }
        }
    }

    /// Creates an optimal instruction set by flattening the instructions, and removing all
    /// operations that are "NoOp"s
    pub fn optimize(vs: Vec<Instruction<T>>) -> Instruction<T> {
        let mut sequence = Self::opt(&vs).flatten();

        let mut fv_map: HashMap<(&str, &T), i32> = HashMap::new();

        for instr in &sequence {
            match instr {
                Instruction::Add(f, v) => {
                    let key = (f.as_str(), v);
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
                    let key = (f.as_str(), v);
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

        sequence.into_iter().fold(Instruction::NoOp, |acc, instr| {
            match instr {
                Instruction::NoOp => acc,
                Instruction::Add(f, v) => {
                    let key = (f.as_str(), v.clone());
                    let i = *fv_map.get(&key).expect("Should be initialized in the previous pass");
                    if i > 0 {
                        fv_map.insert(key, 0);
                        acc.combine(Instruction::Add(f, v))
                    } else {
                        acc
                    }
                }
                Instruction::Rem(f, v) => {
                    let key = (f.as_str(), v.clone());
                    let i = *fv_map.get(&key).expect("Should be initialized in the previous pass");
                    if i < 0 {
                        fv_map.insert(key, 0);
                        acc.combine(Instruction::Rem(f, v))
                    } else {
                        Instruction::NoOp
                    }
                }
                Instruction::Then(..) =>
                    unreachable!("`Then` instruction should never occur in a flattened instruction set")
            }
        })
    }

    // Removes all "NoOp"s from an instruction set
    fn opt(xs: &[Instruction<T>]) -> Instruction<T> {
        match xs {
            [] => Self::NoOp,
            [y, ys @ ..] if matches!(y, Self::NoOp) => Self::opt(ys),
            [y, ys @ ..] => Self::Then(Box::new(y.clone()), Box::new(Self::opt(ys))),
        }
    }
}
