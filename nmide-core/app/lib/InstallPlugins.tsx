import "./Window";
import { pipe } from "fp-ts/lib/function";
import * as M from "fp-ts/Map";
import * as S from "fp-ts/string";
import { useEffect } from "react";
import { NmluginUnknown as Nmlugin } from "@nmide/js-utils";
import NmideClient from "./NmideClient";

export const InstallPlugins = (
  setInstalled: React.Dispatch<React.SetStateAction<boolean>>,
) => useEffect(() => {
  if (window === undefined) return;
  InstallPluginsFunction().then(_ => setInstalled(true))
    .catch(err => console.error("Install Error: ", err));
}, []);

export const InstallPluginsFunction = (): Promise<void> => new Promise(
  // TODO: Should probably reject if its Left<Error>
  resolve => NmideClient("install").then(_ => resolve())
);

export const LoadPlugins = (
  setPlugins: React.Dispatch<React.SetStateAction<[string, Nmlugin][]>>,
  installed: boolean,
) => useEffect(() => {
  if (window === undefined || !installed) return;
  LoadPluginsFunction().then(setPlugins)
  return () => {
    setPlugins([]);
  }
}, [installed]);

export const LoadPluginsFunction = (): Promise<[string, Nmlugin][]> => new Promise(resolve => resolve(
  pipe(
    window.plugins,
    M.toArray(S.Ord),
  )
));
