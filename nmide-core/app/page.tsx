"use client"

import { useEffect, useState } from "react";
import React from "react";
import { THtml } from "./lib/bindings/THtml";
import RenderHtml from "./lib/Html";
import "./lib/Window";
import { v4 as uuidv4 } from "uuid";
import View from "./lib/View";


export default function Page() {
  const [htmls, setHtmls] = useState<THtml[]>([]);

  View(setHtmls);

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

