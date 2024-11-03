import Nmlugin from "./Nmlugin";
import * as E from "fp-ts/Either";
import * as A from "fp-ts/Array";
import * as U from "./Utils";
import ModelFold from "./ModelFold";
import { TMap } from "./bindings/TMap";
import { pipe } from "fp-ts/lib/function";
import { PathReporter } from "io-ts/PathReporter";
import { DMapArr } from "./Decoder";
import { useEffect } from "react";
import NmideClient from "./NmideClient";
import "./Window";
import { PluginMonoid } from "./Utils";

const pluginInit = (p: Nmlugin): TMap => pipe(
  p.init(),
  DMapArr.decode,
  decoded => E.isRight(decoded)
    ? E.right(decoded.right)
    : E.left(
      new Error(
        `Failed to decode model: ${PathReporter.report(decoded).join("\n")}`
      )
    ),
  E.getOrElse<Error, TMap>(err => {
    console.error(err);
    return [];
  }),
);

const Init = (
  plugins: Nmlugin[],
  setModel: React.Dispatch<React.SetStateAction<TMap>>,
) => useEffect(() => {
  const tmodel = pipe(
    plugins,
    A.foldMap(PluginMonoid)(pluginInit),
  );
  NmideClient("init")
    .then(val => {
      if (E.isLeft(val)) {
        console.error("Error from init: ", val.left);
        return;
      }
      setModel(ModelFold(val.right, tmodel));
    });
}, [plugins]);

export default Init;
