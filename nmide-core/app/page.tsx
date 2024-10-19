"use client"

import { useEffect, useState } from "react";
import React from "react";
import { THtml } from "./lib/bindings/THtml";
import RenderHtml from "./lib/Html";
import { v4 as uuidv4 } from "uuid";
import View from "./lib/View";
import NmideClient from "./lib/NmideClient";
import * as E from "fp-ts/Either";

export default function Page() {
  const [htmls, setHtmls] = useState<THtml[]>([]);
  const [listening, setListening] = useState(false);

  useEffect(() => {
    if (!listening) return;
    NmideClient("init", undefined)
      .then(val => {
        if (E.isLeft(val)) {
          console.error("Error from init: ", val.left);
        }
      });
  }, [listening]);
  View(setHtmls, setListening);

  return (
    <>
      {htmls.map((html) => (
        <RenderHtml
          kind={html.kind}
          kids={html.kids}
          text={html.text}
          attrs={html.attrs}
          key={uuidv4()}
        />
      ))}
    </>
  );
}

