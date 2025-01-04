import client from "./lib/NmideClient";
import { emit, listen } from "@tauri-apps/api/event";
import { getPaths } from "./ide/getPaths";
import { InstallHtmlPlugin } from "./ide/htmlInstaller";
import { jspInstaller } from "./lib/jspInstaller";
import { cssInstaller } from "./lib/cssInstaller";
import { App } from "./App";
import { emptyHtml } from "@nmide/js-utils";

// TODO: Add docs
document.addEventListener("DOMContentLoaded", () => {
  App(
    {
      ui: emptyHtml(),
      uiModifications: [],
      state: [],
      stateModifications: [],
      events: [],
      eventModifications: []
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
