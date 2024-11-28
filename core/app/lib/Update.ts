import { pipe } from "fp-ts/lib/function";
import * as E from "fp-ts/Either";
import { PathReporter } from "io-ts/lib/PathReporter";
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
          `Failed to decode model from Plugin: ${pln}`
          + `, supplied model: ${JSON.stringify(p.update(msg, model))}`
          + `, errors: ${PathReporter.report(decoded).join("\n")}`
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
    .then(StateUpdateHandler(plugins.map(PluginUpdate(tmsg, tmodel))))
    .catch(err => { return err; });

