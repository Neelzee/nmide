import {
  AppConfig,
  AppOption,
  defaultConfig,
} from "@nmide/js-utils";
import { InstallPlugins } from "./lib/InstallPlugins";
import "@nmide/js-utils";
import { pipe } from "fp-ts/lib/function";
import * as M from "fp-ts/Map";
import * as S from "fp-ts/string";
import { ModuleUnknown as Module } from "@nmide/js-utils/lib/Module";
import { Core } from "@nmide/js-utils/lib/Core";
import { coreModifications } from "./lib/coreModification";
import { eventHandler } from "./runtime";

// TODO: Add docs
export const App = (core: Core, opts?: AppOption): void => {

  if (opts === undefined) {
    opts = defaultConfig;
  }

  const partialConfig: Partial<AppConfig> = Object.fromEntries(
    Object.entries(opts).filter(([_, v]) => v !== undefined)
  );

  const config: AppConfig = { ...defaultConfig, ...partialConfig };

  window.core = core;
  window.plugins = new Map();
  const originalSet = window.plugins.set.bind(window.plugins);
  window.plugins.set = (key: string, val: Module) => {
    window.moduleCount--;
    return originalSet(key, val);
  };
  window.moduleCount = 0;
  window.pluginAssets = config.pluginAssets;
  window.root = config.root;
  window.listen = config.listen;
  window.emit = config.emit;
  window.log = config.log;
  window.getPluginPaths = config.getPluginPaths; window.pluginInstallers = config.pluginInstallers;
  window.client = config.client;
  window.uiMap = new Map();

  InstallPlugins()
    .catch(err => window.log.error(err))
    .then(() => window.log.info("Installed plugins"))
    .then(() => M.toArray(S.Ord)(window.plugins))
    .then(modules => pipe(
      modules,
      coreModifications,
    ))
    .catch(err => window.log.error(err))
    .then(core => {
      window.log.info("Finished startup");
      return core;
    })
    .catch(err => window.log.error(err))
    .then(core => {
      if (typeof core === "function") {
        return core();
      } else {
        return undefined;
      }
    })
    .then(core => core !== undefined ? window.core = core : window.log.error("No core"))
    .then(core => core !== undefined ? eventHandler() : window.log.error("No eventhandling"))
    .catch(err => window.log.error(err))
};
