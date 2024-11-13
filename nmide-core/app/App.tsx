import { useState } from "react";
import React from "react";
import { THtml, TMap, TMsg, NmluginUnknown as Nmlugin } from "@nmide/js-utils";
import RenderHtml from "./lib/Html";
import { v4 as uuidv4 } from "uuid";
import View from "./lib/View";
import Init from "./lib/Init";
import MsgListener from "./lib/MsgListener";
import { InstallPlugins, LoadPlugins } from "./lib/InstallPlugins"
import Update from "./lib/Update";


export default function App() {
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

