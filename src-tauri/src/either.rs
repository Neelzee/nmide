use crate::errors::NmideError;

#[derive(Debug)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<NmideError<L>, NmideError<R>> {
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

impl<L, R> Either<L, R> {
    pub fn map<F, A, B>(self, f: F) -> Either<A, B>
    where
        F: FnOnce(Either<L, R>) -> Either<A, B>,
    {
        f(self)
    }
}
