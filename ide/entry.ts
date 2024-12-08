import { parseHtml, renderHtml } from "../core/app/lib/renderHtml";
import client from "../core/app/lib/NmideClient";
import { emit, listen } from "@tauri-apps/api/event";
import { getPaths } from "../core/app/ide/getPaths";
import { InstallHtmlPlugin } from "../core/app/ide/htmlInstaller";
import { jspInstaller } from "../core/app/lib/jspInstaller";
import { cssInstaller } from "../core/app/lib/cssInstaller";
import { App } from "../core/app/App";

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

