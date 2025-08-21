export const moduleInitHandler = (
  { init }: UnVerifiedModule
): Either<Error, State> => {
  try {
    const unknown = init();
    const decoded = Decoder.State.decode(unknown);
    return decoded;
  } catch (err) {
    return Left(err);
  }
};
