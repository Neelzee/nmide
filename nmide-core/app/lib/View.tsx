"use client"

import { useEffect } from "react";
import React from "react";
import { THtml } from "./bindings/THtml";
import NmideClient from "./NmideClient";
import { listen } from "@tauri-apps/api/event";
import { pipe } from "fp-ts/lib/function";
import * as E from "fp-ts/Either";
import { TMap } from "./bindings/TMap";

const View = (setHtmls: React.Dispatch<React.SetStateAction<THtml[]>>) => {
  useEffect(() => {
    listen<TMap>(
      "view",
      _ => NmideClient("view", undefined)
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
