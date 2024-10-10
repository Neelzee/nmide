"use client"

import { useEffect } from "react";
import React from "react";
import { THtml } from "./bindings/THtml";
import { pipe } from "fp-ts/lib/function";
import * as E from "fp-ts/Either";
import { TMap } from "./bindings/TMap";
import NmideClient from "./NmideClient";

const BackendView =
  (
    model: TMap,
    setHtmls: React.Dispatch<React.SetStateAction<THtml[]>>
  ): void => useEffect(() => {
    NmideClient("view", { model })
      .then(v => pipe(
        v,
        E.getOrElse<Error, THtml[]>(err => {
          console.error("NmideClient Error:", err);
          return [];
        }),
        setHtmls
      ));
  },
    [model]
  );

export default BackendView;
