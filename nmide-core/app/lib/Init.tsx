import Nmlugin from "./Nmlugin";
import * as E from "fp-ts/Either";
import { TMap } from "nmide-js-utils/bindings/TMap";
import { pipe } from "fp-ts/lib/function";
import { PathReporter } from "io-ts/PathReporter";
import { DMap } from "./Decoder";
import { useEffect } from "react";
import NmideClient from "./NmideClient";
import "./Window";
import { StateUpdateHandler } from "./Utils";

const pluginInit = ([pln, p]: [string, Nmlugin]): [string, TMap] => [
  pln,
  pipe(
    p.init(),
    DMap.decode,
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
  )];

const Init = (
  plugins: [string, Nmlugin][],
  setModel: React.Dispatch<React.SetStateAction<TMap>>,
) => useEffect(() => {
  InitFunction(plugins)
    .then(val => {
      if (E.isLeft(val)) {
        console.error("Error from init: ", val.left);
        return;
      }
      setModel(val.right);
    });
}, [plugins]);

export const InitFunction = (
  plugins: [string, Nmlugin][],
): Promise<E.Either<Error, TMap>> =>
  NmideClient("init")
    .then(StateUpdateHandler(plugins.map(pluginInit)));


export default Init;
