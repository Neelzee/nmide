export const moduleUpdateHandler = (
  { update }: UnVerifiedModule,
  msg: Msg,
  state: State,
): Either<Error, State> => {
  try {
    const unknown = update(msg, state);
    const decoded = Decoder.State.decode(unknown);
    return decoded;
  } catch (err) {
    return Left(err);
  }
};
