"use client"

import { useState } from "react";
import React from "react";
import { THtml } from "./lib/bindings/THtml";
import RenderHtml from "./lib/Html";
import { v4 as uuidv4 } from "uuid";
import View from "./lib/View";
import Init from "./lib/Init";
import Nmlugin from "./lib/Nmlugin";
import MsgListener from "./lib/MsgListener";
import { TMap } from "./lib/bindings/TMap";
import { InstallPlugins, LoadPlugins } from "./lib/InstallPlugins"
import { TMsg } from "./lib/bindings/TMsg";
import Update from "./lib/Update";

export default function Page() {
  const [htmls, setHtmls] = useState<THtml[]>([]);
  const [plugins, setPlugins] = useState<[string, Nmlugin][]>([]);
  const [model, setModel] = useState<TMap>([]);
  const [installed, setInstalled] = useState(false);
  const [msg, setMsg] = useState<TMsg | undefined>()

  MsgListener(setMsg);
  InstallPlugins(setInstalled);
  LoadPlugins(setPlugins, installed);
  Init(plugins, setModel);
  View(setHtmls, plugins, model);
  Update(model, setModel, msg, plugins);

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

