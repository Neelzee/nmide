import { parseHtml, renderHtml } from "./lib/renderHtml";
import client from "./lib/NmideClient";
import { emit, listen } from "@tauri-apps/api/event";
import { getPaths } from "./ide/getPaths";
import { InstallHtmlPlugin } from "./ide/htmlInstaller";
import { jspInstaller } from "./lib/jspInstaller";
import { cssInstaller } from "./lib/cssInstaller";
import { App } from "./App";

document.addEventListener("DOMContentLoaded", () => {
  App({
    cleanup: [],
    pluginAssets: [],
    renderHtml,
    parseHtml,
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

