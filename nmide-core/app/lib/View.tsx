"use client"

import { useEffect } from "react";
import React from "react";
import { THtml } from "./bindings/THtml";
import NmideClient from "./NmideClient";
import { listen } from "@tauri-apps/api/event";
import { pipe } from "fp-ts/lib/function";
import * as E from "fp-ts/Either";

const View = (
  setHtmls: React.Dispatch<React.SetStateAction<THtml[]>>,
  setListening: React.Dispatch<React.SetStateAction<boolean>>
) => {
  useEffect(() => {
    let f = () => { };
    setListening(true);
    listen<void>(
      "view",
      _event => {
        NmideClient("view", undefined)
          .then(v => setHtmls(
            pipe(
              v,
              E.getOrElse<Error, THtml[]>(err => {
                console.error(err);
                return [];
              })
            )
          ));
      }
    )
      .then(g => f = () => {
        g();
      })
      .catch(err => console.error("Error on view listen: ", err));
    return f;
  }, []);
}

export default View;
