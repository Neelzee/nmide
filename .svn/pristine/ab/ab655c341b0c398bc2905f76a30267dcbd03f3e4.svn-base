import {
  THtml,
  TMap,
  Decoder,
  NmluginUnknown as Nmlugin,
  emptyHtml,
} from "@nmide/js-utils";
import { pipe } from "fp-ts/lib/function";
import * as E from "fp-ts/Either";
import * as A from "fp-ts/Array";
import { pluginHandle } from "./pluginHandler";

// TODO: Add docs
export const View = (
  plugins: [string, Nmlugin][],
  tmodel: TMap,
): Promise<[string, THtml][]> =>
  window.client("view", { tmodel })
  .then(xs => pipe(
    plugins,
    A.map(
      ([p, pln]) => pluginHandle<THtml>(
        [p, () => pln.view(tmodel)], Decoder.DHtml.decode
      )
    ),
    A.map<[string, E.Either<Error, THtml>], [string, THtml]>(([pln, e]) => [pln, E.getOrElse<Error, THtml>(r => {
      window.log.error(`Plugin: ${pln}, gave an error on view: `, r);
      return emptyHtml();
    })(e)]),
    A.concat(
      pipe(
        xs,
        E.getOrElse<Error, [string, THtml][]>(err => {
          window.log.error("Error from Client `init` calling backend: ", err);
          return [];
        })
      )
    ),
  ));
