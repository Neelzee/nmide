"use client"

import { useState } from "react";
import React from "react";
import { THtml } from "./lib/bindings/THtml";
import RenderHtml from "./lib/Html";
import { TMap } from "./lib/bindings/TMap";
import Nmlugin from "./lib/Nmlugin";
import Init from "./lib/Init";

export default function Page() {
  const [htmls, _setHtmls] = useState<THtml[]>([]);
  const [_model, setModel] = useState<TMap>({ map: [] });
  const [plugins, _setPlugins] = useState<Nmlugin[]>([]);

  Init(plugins, setModel);

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

