import "@nmide/js-utils";
import { pipe } from "fp-ts/lib/function";
import * as M from "fp-ts/Map";
import * as A from "fp-ts/Array";
import * as E from "fp-ts/Either";
import * as S from "fp-ts/string";
import { AsyncNmluginUnknown, NmluginUnknown as Nmlugin, TMap, TMsg } from "@nmide/js-utils";
import { readDir } from "@tauri-apps/plugin-fs";
import { appDataDir, join } from "@tauri-apps/api/path";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { setTimeout } from "timers/promises";
import NmideClient from "./NmideClient";
import { NmDebugLogMsg } from "@nmide/js-utils/lib/Debug";

export const InstallPlugins = async () => {
  window.plugins = new Map();
  try {
    return await InstallPluginsFunction();
  } catch (err) {
    console.error("Install Error: ", err);
    return [];
  }
};

export const InstallPluginsFunction = async () => {
  const pluginDir = await appDataDir()
    .then(p => join(p, "plugins"));
  return readDir(pluginDir)
    .then(dirs => pipe(
      dirs,
      A.filter(d => d.isFile),
      A.filter(d => d.name.endsWith(".js") || d.name.endsWith(".css")),
    ))
    .then(A.map(p => join(pluginDir, p.name)))
    .then(A.map(p => p.then(path => convertFileSrc(path))))
    .then(paths => Promise.all(paths))
    .then(A.sort(S.Ord))
    .then(A.map(src => {
      let element: HTMLElement;
      if (src.endsWith(".module.js")) {
        const script = document.createElement("script");
        script.src = src;
        script.type = "module"
        element = script;
      } else if (src.endsWith(".js")) {
        const script = document.createElement("script");
        script.src = src;
        element = script;
      } else {
        const style = document.createElement("link");
        style.href = src;
        style.rel = "stylesheet";
        element = style;
      }
      document.head.append(element);
      return () => { document.head.removeChild(element) }
    }))
    .then(paths => setTimeout(250, paths));
}

export const LoadPlugins = (): Promise<[string, Nmlugin][]> => LoadPluginsFunction()
  .catch(err => {
    console.error("Error in LoadPlugins: ", err);
    return [];
  });

export const LoadPluginsFunction = (): Promise<[string, Nmlugin][]> =>
  new Promise(resolve => resolve(
    pipe(
      window.plugins,
      M.toArray(S.Ord),
    )
  ));

export const InstallBackendPluginsFunction = () =>
  NmideClient("get_plugins")
    .then(E.map(A.map<string, [string, AsyncNmluginUnknown]>(pluginName => {
      return [
        pluginName,
        {
          init: () => {
            return invoke("plugin_init", { pluginName });
          },
          update: (tmsg: TMsg, tmodel: TMap) => invoke("plugin_update", { pluginName, tmsg, tmodel }),
          view: (tmodel: TMap) => invoke("plugin_view", { pluginName, tmodel }),
        }
      ];
    })));

export const InstallBackendPlugins = (
) => {
  InstallBackendPluginsFunction()
    .then(result => pipe(
      result,
      NmDebugLogMsg("Installing Backend Plugins"),
      E.getOrElse<Error, [string, AsyncNmluginUnknown][]>(err => {
        console.debug("Error installing backend plugins: ", err);
        return [];
      }),
      plugins => {
        window.async_plugins = new Map();
        plugins.forEach(([k, v]) => window.async_plugins.set(k, v))
      },
    ))
}
