import * as E from "fp-ts/Either";
import { TMap } from "@nmide/js-utils";
import { NmluginUnknown as Nmlugin, StateUpdateHandler } from "@nmide/js-utils";
import "@nmide/js-utils";
import { pluginInit } from "./pluginHandler";


export const Init = (
  plugins: [string, Nmlugin][],
): Promise<E.Either<Error, [TMap, [string, TMap][]]>> =>
  window.client("init")
    .then(StateUpdateHandler(plugins.map(pluginInit)));


