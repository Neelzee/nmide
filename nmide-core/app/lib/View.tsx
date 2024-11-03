"use client"

import { useEffect } from "react";
import React from "react";
import { THtml } from "./bindings/THtml";
import NmideClient from "./NmideClient";
import { pipe } from "fp-ts/lib/function";
import * as E from "fp-ts/Either";
import * as A from "fp-ts/Array";
import Nmlugin from "./Nmlugin";
import { DHtml } from "./Decoder";
import { TMap } from "./bindings/TMap";
import { PathReporter } from "io-ts/PathReporter";

const pluginView = (model: TMap): ((p: Nmlugin) => THtml) => (p: Nmlugin) =>
  pipe(
    p.view(model),
    DHtml.decode,
    decoded => E.isRight(decoded)
      ? E.right(decoded.right)
      : E.left(
        new Error(
          `Failed to decode model: ${PathReporter.report(decoded).join("\n")}`
        )
      ),
    E.getOrElse<Error, THtml>(err => {
      console.error("Error on pluginView: ", err);
      return { kind: "Frag", kids: [], text: null, attrs: [] };
    })
  );

const View = (
  setHtmls: React.Dispatch<React.SetStateAction<THtml[]>>,
  plugins: Nmlugin[],
  tmodel: TMap,
) => {
  useEffect(() => {
    NmideClient("view", { tmodel })
      .then(v => setHtmls(
        pipe(
          v,
          E.getOrElse<Error, THtml[]>(err => {
            console.error("Error from NmideClient in View: ", err);
            return [];
          }),
          A.concat(A.map(pluginView(tmodel))(plugins)),
        )
      ));
  }, [plugins, tmodel]);
}

export default View;
