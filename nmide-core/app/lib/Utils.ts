import * as E from "fp-ts/Either";

export const NmDebugLog = <T>(t: T): T => {
  console.debug("Debug: ", t);
  return t;
};

export const GetOrElse = <R>(t: R): ((v: E.Either<Error, R>) => R) => E.getOrElse<Error, R>(e => {
  console.error("Error: ", e);
  return t;
});
