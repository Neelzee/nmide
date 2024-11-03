"use client"

import "./Window";
import { pipe } from "fp-ts/lib/function";
import * as A from "fp-ts/Array";
import * as T from "fp-ts/Tuple";
import * as M from "fp-ts/Map";
import * as S from "fp-ts/string";
import { useEffect } from "react";
import Nmlugin from "./Nmlugin";
import NmideClient from "./NmideClient";

export const InstallPlugins = (
  setInstalled: React.Dispatch<React.SetStateAction<boolean>>,
) => useEffect(() => {
  if (window === undefined) return;
  NmideClient("install").then(_ => setInstalled(true))
    .catch(err => console.error("Install Error: ", err));
}, []);

export const LoadPlugins = (
  setPlugins: React.Dispatch<React.SetStateAction<Nmlugin[]>>,
  installed: boolean,
) => useEffect(() => {
  if (window === undefined || !installed) return;
  setPlugins(pipe(
    window.plugins,
    M.toArray(S.Ord),
    A.map(T.snd),
  ));
  return () => {
    setPlugins([]);
  }
}, [installed]);
