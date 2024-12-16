import * as E from "fp-ts/Either";
import { TMap } from "@nmide/js-utils";
import { pipe } from "fp-ts/lib/function";
import { PathReporter } from "io-ts/PathReporter";
import { Decoder, NmluginUnknown as Nmlugin } from "@nmide/js-utils";

export const pluginInit = (
  [pln, p]: [string, Nmlugin]
): [string, E.Either<Error, TMap>] =>
  [
    pln,
    pipe(
      tryPluginInit(p),
      E.match(
        E.left,
        u => pipe(
          u,
          Decoder.DMap.decode,
          decoded => E.isRight(decoded)
            ? E.right(decoded.right)
            : E.left(
              new Error(
                `Failed to decode model, plugin: ${pln}`
                + `, supplied model: ${JSON.stringify(p.init())}`
                + `, errors: ${PathReporter.report(decoded).join("\n")}`
              )
            ),
        )
      ),
    )
  ];

const tryPluginInit = (pln: Nmlugin): E.Either<Error, unknown> => {
  try {
    return E.right(pln.init());
  } catch (err) {
    return E.left(new Error(`PluginInit Error: ${err}`));
  };
}
