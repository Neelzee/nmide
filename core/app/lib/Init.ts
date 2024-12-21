import * as E from "fp-ts/Either";
import * as A from "fp-ts/Array";
import * as S from "fp-ts/string";
import { ModelOverwrite, TMap } from "@nmide/js-utils";
import { NmluginUnknown as Nmlugin } from "@nmide/js-utils";
import "@nmide/js-utils";
import { pluginHandle } from "./pluginHandler";
import { pipe } from "fp-ts/lib/function";
import { snd } from "fp-ts/Tuple";
import { fromEquals } from "fp-ts/Eq";
import { Decoder } from "@nmide/js-utils";
import { stateHandler } from "./stateManagement";


// TODO: Add docs
// TODO: Refactor this to use the same code as Update
export const Init = (
  plugins: [string, Nmlugin][],
): Promise<TMap> =>
  window.client("init")
  .then(xs => pipe(
    stateHandler(
      pipe(
        plugins,
        A.map(
          ([p, pln]) => pluginHandle<TMap>(
            [p, () => pln.init()], Decoder.DMap.decode
          )
        ),
        A.map<[string, E.Either<Error, TMap>], [string, TMap]>(([pln, e]) => [pln, E.getOrElse<Error, TMap>(r => {
          window.log.error(`Plugin: ${pln}, gave an error on init: `, r);
          return [];
        })(e)]),
        A.concat(
          pipe(
            xs,
            E.getOrElse<Error, [string, TMap][]>(err => {
              window.log.error("Error from Client `init` calling backend: ", err);
              return [];
            })
          )
        ),
      )
    ),
    ys => pipe(
      window.coalcePluginState,
      A.map(f => f(ys)),
    ),
    A.flatten,
    A.uniq<[string, TMap]>(fromEquals(([a, _], [b, __]) => S.Ord.equals(a, b))),
    A.map(snd),
    A.reduce([], ModelOverwrite),
  ));
