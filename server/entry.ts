import { App } from "../core/app/App";
import { renderHtml, parseHtml } from "../core/app/lib/renderHtml";
import { jspInstaller } from "../core/app/lib/jspInstaller";
import { cssInstaller } from "../core/app/lib/cssInstaller";
import * as E from "fp-ts/Either";

const listenHandler: Map<string, any> = new Map();

document.addEventListener("DOMContentLoaded", () => {
  App({
    renderHtml,
    parseHtml,
    client: (x, args) => {
      if (x === "init") return new Promise(r => r(E.right([])));
      if (x === "view") return new Promise(r => r(E.right([])));
      if (x === "update") return new Promise(r => r(E.right([])));
      console.error("No client for: ", x, args);
      return new Promise(r => r(E.right([])));
    },
    listen: (x, handler) => {
      listenHandler.set(x, handler);
      return new Promise(r => r(null));
    },
    emit: (evt, payload) => {
      const handler = listenHandler.get(evt);
      if (handler !== undefined) {
        handler({ payload });
      }
      return new Promise(r => r());
    },
    getPluginPaths: new Promise(r => r([
      "./plugins/counter.js",
    ])),
    pluginInstallers: [jspInstaller, cssInstaller]
  });
})

