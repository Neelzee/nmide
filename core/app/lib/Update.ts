import { pipe } from "fp-ts/lib/function";
import * as E from "fp-ts/Either";
import { PathReporter } from "io-ts/lib/PathReporter";
import { stateHandler } from "../../output/State";
import { decodeState } from "../../output/TMap";
import {
  TMsg,
  TMap,
  Decoder,
  GetOrElse,
  NmluginUnknown as Nmlugin,
  StateUpdateHandler,
} from "@nmide/js-utils";
import NmideClient from "./NmideClient";

const PluginUpdate = (
  msg: TMsg,
  model: TMap
): (([pln, p]: [string, Nmlugin]) => [string, TMap]) =>
  ([pln, p]: [string, Nmlugin]) => pipe(
    p.update(msg, model),
    Decoder.DMap.decode,
    decoded => E.isRight(decoded)
      ? E.right(decoded.right)
      : E.left(
        new Error(
          `Failed to decode model from Plugin: ${PathReporter.report(decoded).join("\n")}`
        )
      ),
    GetOrElse<TMap>([]),
    model => [pln, model],
  );

export const Update = (
  tmodel: TMap,
  tmsg: TMsg,
  plugins: [string, Nmlugin][],
) => UpdateFunction(tmsg, plugins, tmodel);


export const UpdateFunction = (
  tmsg: TMsg,
  plugins: [string, Nmlugin][],
  tmodel: TMap,
): Promise<E.Either<Error, [TMap, [string, TMap][]]>> =>
  NmideClient("update", { tmsg, tmodel })
    .then(val => {
      const pluginModel = plugins.map(PluginUpdate(tmsg, tmodel));
      if (E.isRight(val)) {
        console.log("pluginModel: ", pluginModel);
        try {
          const st = decodeState({ tmap: pluginModel });
          console.log(st)
          const f = stateHandler(st);
          console.log("foo: ", f);
        } catch (err) {
          console.log("err: ", err);
        }
      }
      return StateUpdateHandler(pluginModel)(val);
    })
    .catch(err => { return err; });

