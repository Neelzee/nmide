"use client"

import { TMap } from "./bindings/TMap"
import { pipe } from "fp-ts/lib/function";
import * as E from "fp-ts/Either";
import { PathReporter } from "io-ts/lib/PathReporter";
import * as U from "./Utils";
import * as A from "fp-ts/Array";
import { useEffect } from "react";
import { TMsg } from "./bindings/TMsg";
import Nmlugin from "./Nmlugin";
import { DMapArr } from "./Decoder";
import NmideClient from "./NmideClient";
import ModelFold from "./ModelFold";

const PluginUpdate = (msg: TMsg, model: TMap): ((p: Nmlugin) => TMap) =>
  (p: Nmlugin) => pipe(
    p,
    pl => pl.update(msg, model),
    DMapArr.decode,
    decoded => E.isRight(decoded)
      ? E.right(decoded.right)
      : E.left(
        new Error(
          `Failed to decode model from Plugin: ${PathReporter.report(decoded).join("\n")}`
        )
      ),
    U.GetOrElse<TMap>([]),
  );

const Update = (
  tmodel: TMap,
  setModel: React.Dispatch<React.SetStateAction<TMap>>,
  tmsg: TMsg | undefined,
  plugins: Nmlugin[],
): void => {
  useEffect(() => {
    if (tmsg === undefined) return;
    const model = pipe(
      plugins,
      A.foldMap(U.PluginMonoid)(PluginUpdate(tmsg, tmodel)),
    );
    NmideClient("update", { tmsg, tmodel })
      .then(val => {
        if (E.isLeft(val)) {
          console.error("Error on update: ", val.left);
        } else {
          setModel(ModelFold(val.right, model));
        }
      })
  }, [tmsg]);
};

export default Update;
