"use client"

import { listen } from "@tauri-apps/api/event"
import { TMap } from "./bindings/TMap"
import { DMap } from "./Decoder";
import { pipe } from "fp-ts/lib/function";
import * as E from "fp-ts/Either";
import { PathReporter } from "io-ts/lib/PathReporter";
import * as U from "./Utils";

export const Update = (setModel: React.Dispatch<React.SetStateAction<TMap>>): void => {
  listen("update", ({ payload }) => {
    setModel(
      pipe(
        payload,
        DMap.decode,
        decoded => E.isLeft(decoded)
          ? E.left(
            new Error(
              `Failed to decode html from js-plugin: ${PathReporter
                .report(decoded)
                .join("\n")}`
            )
          )
          : E.right(decoded.right),
        U.GetOrElse<TMap>([]),
      )
    );
  })
};
