export const moduleViewHandler = (
  { view }: UnVerifiedModule,
  state: State,
): Either<Error, State> => {
  try {
    const unknown = view(msg, state);
    const decoded = Decoder.HTML.decode(unknown);
    return decoded;
  } catch (err) {
    return Left(err);
  }
};
