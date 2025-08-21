import client from "./lib/NmideClient";
import { emit, listen } from "@tauri-apps/api/event";
import { getPaths } from "./ide/getPaths";
import { InstallHtmlPlugin } from "./ide/htmlInstaller";
import { jspInstaller } from "./lib/jspInstaller";
import { cssInstaller } from "./lib/cssInstaller";
import { App } from "./App";
import { emptyHtml, tObj } from "@nmide/js-utils";
import { toNode } from "@nmide/js-utils/lib/tree";

// TODO: Add docs
document.addEventListener("DOMContentLoaded", () => {
  App(
    {
      ui: toNode(emptyHtml()),
      state: { id: "root", kids: [], ...tObj([]), },
      events: { id: "root", kids: [], event: "", module: "", },
      eventThrower: evt => {
        emit(evt.event, evt);
      },
      eventHandlers: new Map(),
    },
    {
      pluginAssets: [],
      root: document.body,
      client,
      log: {
        info: console.log,
        error: console.error,
      },
      listen,
      emit,
      getPluginPaths: getPaths(),
      pluginInstallers: [InstallHtmlPlugin, jspInstaller, cssInstaller],
    });
})
