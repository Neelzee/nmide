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

const View = (setHtmls: React.Dispatch<React.SetStateAction<THtml[]>>) => {
  useEffect(() => {
    listen<TMap>(
      "view",
      ({ payload: model }) => NmideClient("view", { model })
        .then(v => setHtmls(
          pipe(
            v,
            E.getOrElse<Error, THtml[]>(err => {
              console.error(err);
              return [];
            })
          )
        ))
    )
  }, []);
}

export default View;
