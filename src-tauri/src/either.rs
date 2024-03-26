use crate::errors::NmideError;

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl Either<NmideError<L>, NmideError<R>> {
    pub fn transpose(self) -> NmideError<Either<L, R>> {
        match self {
            Either::Left(e) => {
                e.
                todo!();
                todo!();
            }
            Either::Right(e) => todo!(),
        }
    }
}
