import { cssInstaller } from "./lib/cssInstaller";
import { jspInstaller } from "./lib/jspInstaller";
import { parseHtml, renderHtml } from "./lib/renderHtml";

const defaultConfig = {
  cleanup: [],
  pluginAssets: [],
  renderHtml,
  parseHtml,
  root: document.body,
  log: {
    info: console.log,
    error: console.error,
  },
  listen: (_: any, __: any) => new Promise(r => r(undefined)),
  emit: (_: any, __?: any) => new Promise(r => r(undefined)),
  client: (..._: any) => new Promise(r => r({})),
  getPluginPaths: new Promise(r => r([])),
  pluginInstallers: [cssInstaller, jspInstaller],
};

export const setup = (opts?: any) => {
  if (opts === undefined) {
    opts = defaultConfig;
  }
  window.plugins = new Map();
  window.cleanup = opts.cleanup === undefined ? defaultConfig.cleanup : opts.cleanup;
  window.pluginAssets = opts.pluginAssets === undefined ? defaultConfig.pluginAssets : opts.pluginAssets;
  window.renderHtml = opts.renderHtml === undefined ? defaultConfig.renderHtml : opts.renderHtml;
  window.parseHtml = opts.parseHtml === undefined ? defaultConfig.parseHtml : opts.parseHtml;
  window.root = opts.root === undefined ? defaultConfig.root : opts.root;
  window.listen = opts.listen === undefined ? defaultConfig.listen : opts.listen;
  window.emit = opts.emit === undefined ? defaultConfig.emit : opts.emit;
  window.log = opts.log === undefined ? defaultConfig.log : opts.log;
  window.getPluginPaths = opts.getPluginPaths === undefined ? defaultConfig.getPluginPaths : opts.getPluginPaths;
  window.pluginInstallers = opts.pluginInstallers === undefined ? defaultConfig.pluginInstallers : opts.pluginInstallers;
  window.client = opts.client === undefined ? defaultConfig.client : opts.client;
};

