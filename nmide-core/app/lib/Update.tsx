import { pipe } from "fp-ts/lib/function";
import * as E from "fp-ts/Either";
import { PathReporter } from "io-ts/lib/PathReporter";
import { useEffect } from "react";
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

const Update = (
  tmodel: TMap,
  setModel: React.Dispatch<React.SetStateAction<TMap>>,
  tmsg: TMsg | undefined,
  plugins: [string, Nmlugin][],
): void => {
  useEffect(() => {
    if (tmsg === undefined) return;
    UpdateFunction(tmsg, plugins, tmodel)
      .then(val => {
        if (E.isLeft(val)) {
          console.error("Error on update: ", val.left);
        } else {
          setModel(val.right[0]);
        }
      })
  }, [tmsg]);
};

export const UpdateFunction = (
  tmsg: TMsg,
  plugins: [string, Nmlugin][],
  tmodel: TMap,
): Promise<E.Either<Error, [TMap, [string, TMap][]]>> =>
  NmideClient("update", { tmsg, tmodel })
    .then(StateUpdateHandler(plugins.map(PluginUpdate(tmsg, tmodel))))
    .catch(err => { return err; });

export default Update;
