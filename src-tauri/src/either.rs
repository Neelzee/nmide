use crate::errors::NmideError;

#[derive(Debug, Clone)]
pub enum Either<L: Clone, R: Clone> {
    Left(L),
    Right(R),
}

impl<L: Clone, R: Clone> Either<NmideError<L>, NmideError<R>> {
    pub fn transpose(self) -> NmideError<Either<L, R>> {
        match self {
            Either::Left(e) => NmideError {
                val: Either::Left(e.val),
                rep: e.rep,
            },
            Either::Right(e) => NmideError {
                val: Either::Right(e.val),
                rep: e.rep,
            },
        }
    }
}

impl<L: Clone, R: Clone> Either<L, R> {
    pub fn map<F, A: Clone, B: Clone>(self, f: F) -> Either<A, B>
    where
        F: FnOnce(Either<L, R>) -> Either<A, B>,
    {
        f(self)
    }
}
