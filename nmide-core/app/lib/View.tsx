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
    console.debug("LISTENING");
    let f = () => {
      console.debug("UNLISTEN");
    };
    listen<void>(
      "view",
      _event => {
        console.debug("Event: ", _event);
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
        console.debug("UNLISTEN");
        g();
      })
      .catch(err => console.error("Error on view listen: ", err));
    return f;
  }, []);
}

export default View;
