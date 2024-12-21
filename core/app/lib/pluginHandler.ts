import * as E from "fp-ts/Either";
import { pipe } from "fp-ts/lib/function";
import { PathReporter } from "io-ts/PathReporter";
import { Validation } from "io-ts";


// TODO: Add docs
const tryPluginCallback = (pln: (() => unknown)): E.Either<Error, unknown> => {
  try {
    return E.right(pln());
  } catch (err) {
    return E.left(new Error(`PluginInit Error: ${err}`));
  };
};


// TODO: Add docs
export const pluginHandle = <T>([pln, p]: [string, (() => unknown)], decode: ((u: unknown) => Validation<T>)): [string, E.Either<Error, T>] => pipe(
  tryPluginCallback(p),
  E.match(
    E.left,
    u => pipe(
      u,
      decode,
      decoded => E.isRight(decoded)
        ? E.right(decoded.right)
        : E.left(
          new Error(
            `Failed to decode value, plugin: ${pln}`
              + `, supplied value ${JSON.stringify(u)}`
              + `, errors: ${PathReporter.report(decoded).join("\n")}`
          )
        ),
    )
  ),
  d => [pln, d]
);
