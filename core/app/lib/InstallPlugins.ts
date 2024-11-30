import "@nmide/js-utils";
import { pipe } from "fp-ts/lib/function";
import * as M from "fp-ts/Map";
import * as A from "fp-ts/Array";
import * as E from "fp-ts/Either";
import * as S from "fp-ts/string";
import * as O from "fp-ts/Ord";
import { AsyncNmluginUnknown, NmluginUnknown as Nmlugin, TMap, TMsg } from "@nmide/js-utils";
import { DirEntry, readDir } from "@tauri-apps/plugin-fs";
import { appDataDir, join } from "@tauri-apps/api/path";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { setTimeout } from "timers/promises";
import NmideClient from "./NmideClient";
import { NmDebugLogMsg } from "@nmide/js-utils/lib/Debug";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

export const InstallPlugins = async () => {
  window.plugins = new Map();
  try {
    await InstallHtmlPlugin();
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
    .then(A.map<DirEntry, Promise<[string, string]>>(p => join(pluginDir, p.name).then(path => [p.name, path])))
    .then(A.map<Promise<[string, string]>, Promise<[string, string]>>(p => p.then(([pln, path]) => [pln, convertFileSrc(path)])))
    .then(paths => Promise.all(paths))
    .then(A.sort(O.fromCompare<[string, string]>(([a, _], [b, __]) => S.Ord.compare(a, b))))
    .then(A.map(([pln, src]) => {
      window.pluginAssets.push([pln, src]);
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
};

export const InstallHtmlPlugin = async () => {
  const pluginDir = await appDataDir()
    .then(p => join(p, "plugins"));
  return readDir(pluginDir)
    .then(dirs => pipe(
      dirs,
      A.filter(d => d.isFile),
      A.filter(d => d.name.endsWith(".wb.html")),
    ))
    .then(A.map<DirEntry, Promise<[string, string]>>(p => join(pluginDir, p.name).then(path => [p.name, path])))
    .then(A.map<Promise<[string, string]>, Promise<[string, string]>>(p => p.then(([pln, path]) => [pln, convertFileSrc(path)])))
    .then(paths => Promise.all(paths))
    .then(A.sort(O.fromCompare<[string, string]>(([a, _], [b, __]) => S.Ord.compare(a, b))))
    .then(A.map(([pln, url]) => {
      window.pluginAssets.push([pln, url]);
      let name = url.split("%2F").pop()?.split(".")[0];
      name = name === undefined ? url : name
      const wb = new WebviewWindow(`nmide-${name}`, { url });
      wb.once("tauri://webview-created", () => {
        wb.setTitle(name)
          .then(_ => _);
      });
      wb.once("tauri://error", err => {
        console.error(`Error in creating webview: ${name}: `, err);
      });
    }));
};

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
