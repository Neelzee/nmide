"use client"

import { useState } from "react";
import React from "react";
import { THtml } from "./lib/bindings/THtml";
import RenderHtml from "./lib/Html";

export default function Page() {
  const [htmls, _] = useState<THtml[]>([]);

  return (
    <>
      {htmls.map((html) => (
        <RenderHtml
          kind={html.kind}
          kids={html.kids}
          text={html.text}
          attrs={html.attrs}
        />
      ))}
    </>
  );
}

