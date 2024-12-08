import {
  THtml,
  TMap,
  Decoder,
  NmluginUnknown as Nmlugin,
  emptyHtml
} from "@nmide/js-utils";
import { pipe } from "fp-ts/lib/function";
import * as E from "fp-ts/Either";
import * as A from "fp-ts/Array";
import { PathReporter } from "io-ts/PathReporter";

const pluginView = (
  model: TMap
): (([pln, p]: [string, Nmlugin]) => [string, THtml]) => ([pln, p]) =>
    pipe(
      p.view(model),
      Decoder.DHtml.decode,
      decoded => E.isRight(decoded)
        ? E.right(decoded.right)
        : E.left(
          new Error(
            `Failed to decode plugin: ${pln}'s`
            + `, supplied view: ${JSON.stringify(p.view(model))}`
            + `, errors: ${PathReporter.report(decoded).join("\n")}`
          )
        ),
      E.getOrElse<Error, THtml>(err => {
        console.error("Error on pluginView: ", err);
        return { kind: "frag", kids: [], text: null, attrs: [] };
      }),
      h => [pln, h]
    );

export const View = (
  plugins: [string, Nmlugin][],
  tmodel: TMap,
) =>
  window.client("view", { tmodel })
    .then(v => pipe(
      v,
      E.getOrElse<Error, [string, THtml][]>(err => {
        console.error("Error from NmideClient in View: ", err);
        return [];
      }),
      A.concat(A.map<[string, Nmlugin], [string, THtml]>(plugin => {
        try {
          return pluginView(tmodel)(plugin);
        } catch (err) {
          window.log.error(
            `Got error on plugin: ${plugin[0]}, error: ${JSON.stringify(err)}`
          );
          return [plugin[0], emptyHtml()];
        }
      })(plugins)),
    ));

