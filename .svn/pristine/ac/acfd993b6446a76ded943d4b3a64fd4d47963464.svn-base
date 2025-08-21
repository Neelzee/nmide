import { pipe } from "fp-ts/lib/function";
import * as E from "fp-ts/Either";
import * as S from "fp-ts/string";
import * as A from "fp-ts/Array";
import { fromEquals } from "fp-ts/Eq";
import { snd } from "fp-ts/Tuple";
import {
  TMsg,
  TMap,
  NmluginUnknown as Nmlugin,
  ModelOverwrite,
} from "@nmide/js-utils";
import { pluginHandle } from "./pluginHandler";
import { Decoder } from "@nmide/js-utils";
import { stateHandler } from "./stateManagement";

// TODO: Add docs
// TODO: Refactor this to use the same code as Init
// Maybe move the error-handling to the client?
export const Update = (
  tmsg: TMsg,
  plugins: [string, Nmlugin][],
  tmodel: TMap,
): Promise<TMap> =>
  window.client("update", { tmsg, tmodel })
  .then(xs => pipe(
    stateHandler(
      pipe(
        plugins,
        A.map(
          ([p, pln]) => pluginHandle<TMap>(
            [p, () => pln.update(tmsg, tmodel)], Decoder.DMap.decode
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
              window.log.error("Error from Client calling `update` backend: ", err);
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
