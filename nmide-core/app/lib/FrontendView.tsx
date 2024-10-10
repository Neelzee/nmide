"use client"

import { useEffect } from "react";
import React from "react";
import { THtml } from "./bindings/THtml";
import Nmlugin from "./Nmlugin";
import { pipe } from "fp-ts/lib/function";
import * as A from "fp-ts/Array";
import * as E from "fp-ts/Either";
import * as O from "fp-ts/Option";
import { DHtml } from "./Decoder";
import { PathReporter } from "io-ts/lib/PathReporter";
import { TMap } from "./bindings/TMap";

const FrontendView =
  (
    nmlugs: Nmlugin[],
    model: TMap,
    setHtmls: React.Dispatch<React.SetStateAction<THtml[]>>
  ): void => useEffect(() => setHtmls(
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
                `Failed to decode html from js-plugin: ${PathReporter
                  .report(decoded)
                  .join("\n")}`
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
      )
    )
  ),
    [nmlugs, model]
  );

export default FrontendView;
