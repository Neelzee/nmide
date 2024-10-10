"use client"

import { useEffect, useState } from "react";
import React from "react";
import { THtml } from "./lib/bindings/THtml";
import RenderHtml from "./lib/Html";
import Nmlugin from "./lib/Nmlugin";
import Init from "./lib/Init";
import NmideClient from "./lib/NmideClient";
import "./lib/Window";
import * as E from "fp-ts/Either";
import { TMap } from "./lib/bindings/TMap";
import FrontendView from "./lib/FrontendView";
import BackendView from "./lib/BackendView";
import { Update } from "./lib/Update";
import { v4 as uuidv4 } from "uuid";


export default function Page() {
  const [htmls, setHtmls] = useState<THtml[]>([]);
  const [frontendHtmls, setFrontendHtmls] = useState<THtml[]>([]);
  const [backendHtmls, setBackendHtmls] = useState<THtml[]>([]);
  const [model, setModel] = useState<TMap>([]);
  const [plugins, setPlugins] = useState<Nmlugin[]>([]);

  useEffect(() => {
    if (window === undefined) return () => { };
    NmideClient("install_plugins", undefined)
      .then(res => {
        if (E.isLeft(res)) {
          console.error(res.left);
        } else {
          const nmlugs = window.plugins;
          setPlugins(nmlugs);
          Init(nmlugs);
        }
      })
      .catch(err => console.error(err));
    return () => {
      NmideClient("uninstall_plugins", undefined)
        .then(res => {
          if (E.isLeft(res)) console.error(res.left);
          setPlugins([]);
        })
        .catch(err => console.error(err));
    };
  }, []);

  FrontendView(plugins, model, setFrontendHtmls);
  BackendView(model, setBackendHtmls);
  useEffect(() => setHtmls(frontendHtmls.concat(backendHtmls)), [frontendHtmls, backendHtmls]);
  Update(setModel);


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

