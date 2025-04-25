use crate::instruction::inst::Instruction;

impl<T: PartialEq + Clone> Instruction<T> {
    pub fn is_noop(&self) -> bool {
        matches!(self, Self::NoOp)
    }

    pub fn optimize(vs: Vec<Instruction<T>>) -> Vec<Instruction<T>> {
        todo!()
    }

    fn rems(xs: &[Instruction<T>]) -> Vec<(Some(String), Some(String))> {
        xs.into_iter().filter_map(|i| {
            match i {
                Instruction::Rem(a, b, _) => todo!(),
                _ => None,
            }
        }).collect()
    }

    fn opt(xs: &[Instruction<T>]) -> Instruction<T> {
        match xs {
            [] => Self::NoOp,
            [y, ys@..] if y.is_noop() => Self::opt(ys),
            [y, ys@..] =>
                Self::Then(Box::new(y.clone()), Box::new(Self::opt(ys))),
        }
    }
}
