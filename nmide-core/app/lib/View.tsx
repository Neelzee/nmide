"use client"

import { useEffect } from "react";
import React from "react";
import { THtml } from "./bindings/THtml";
import Nmlugin from "./Nmlugin";
import NmideClient from "./NmideClient";
import { listen } from "@tauri-apps/api/event";
import { pipe } from "fp-ts/lib/function";
import * as A from "fp-ts/Array";
import * as E from "fp-ts/Either";
import * as O from "fp-ts/Option";
import { DHtml } from "./Decoder";
import { PathReporter } from "io-ts/lib/PathReporter";
import { TMap } from "./bindings/TMap";

const View = (nmlugs: Nmlugin[], setHtmls: React.Dispatch<React.SetStateAction<THtml[]>>) => {
  useEffect(() => {
    //console.log("I HAVE PLUGINS!!!", nmlugs);
    listen<TMap>(
      "view",
      ({ payload: model }) => NmideClient("view", { model })
        .then(v => setHtmls(
          pipe(
            nmlugs,
            A.filterMap(
              p => pipe(
                p.view(model),
                DHtml.decode,
                decoded => E.isRight(decoded)
                  ? E.right(decoded.right)
                  : E.left(
                    new Error(
                      `Failed to decode html: ${PathReporter.report(decoded).join("\n")}`
                    )
                  ),
                E.match(
                  e => {
                    console.error(e);
                    return O.none;
                  },
                  h => O.some(h)
                ),
              )
            ),
            A.concat(
              pipe(
                v,
                E.getOrElse<Error, THtml[]>(err => {
                  console.error(err);
                  return [];
                }),
              )
            )
          )
        ))
    )
  }, [nmlugs]);
}

export default View;
