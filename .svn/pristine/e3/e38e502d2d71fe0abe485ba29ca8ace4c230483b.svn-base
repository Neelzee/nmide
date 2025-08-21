import * as E from "fp-ts/Either";
import { pipe } from "fp-ts/lib/function";


// TODO: Add docs
const tryModuleCallback = (pln: (() => Promise<unknown>)): E.Either<Error, Promise<unknown>> => {
  try {
    return E.right(pln());
  } catch (err) {
    return E.left(new Error(`ModuleInit Error: ${err}`));
  }
};


// TODO: Add docs
export const moduleHandle = <T>(
  [pln, p]: [string, (() => Promise<unknown>)],
): [string, E.Either<Error, Promise<T>>] => pipe(
  tryModuleCallback(p),
  // TODO: Add actual decoding of returned value
  d => [pln, d as E.Either<Error, Promise<T>>],
);
