import {
  AppConfig,
  AppOption,
  defaultConfig,
} from "@nmide/js-utils";
import { InstallPlugins } from "./lib/InstallPlugins";
import "@nmide/js-utils";
import { pipe } from "fp-ts/lib/function";
import * as M from "fp-ts/Map";
import * as O from "fp-ts/Option";
import * as A from "fp-ts/Array";
import * as S from "fp-ts/string";
import * as TE from "fp-ts/TaskEither";
import { Core, ModuleUnknown as Module } from "@nmide/js-utils/lib/Module";

// TODO: Add docs
export const App = (core: Core, opts?: AppOption): void => {

  if (opts === undefined) {
    opts = defaultConfig;
  }

  const partialConfig: Partial<AppConfig> = Object.fromEntries(
    Object.entries(opts).filter(([_, v]) => v !== undefined)
  );

  const config: AppConfig = { ...defaultConfig, ...partialConfig };

  window.plugins = new Map();
  const originalSet = window.plugins.set.bind(window.plugins);
  window.plugins.set = (key: string, val: Module) => {
    window.moduleCount--;
    return originalSet(key, val);
  };
  window.moduleCount = 0;
  window.cleanup = config.cleanup;
  window.pluginAssets = config.pluginAssets;
  window.renderHtml = config.renderHtml;
  window.parseHtml = config.parseHtml;
  window.root = config.root;
  window.listen = config.listen;
  window.emit = config.emit;
  window.log = config.log;
  window.getPluginPaths = config.getPluginPaths;
  window.pluginInstallers = config.pluginInstallers;
  window.client = config.client;
  window.coalcePluginState = config.coalcePluginState;

  InstallPlugins()
    .then(() => M.toArray(S.Ord)(window.plugins))
    .then(modules => pipe(
      modules,
      A.map(([moduleName, module]) => pipe(
        TE.tryCatch<Error, unknown>(
          // HACK: Modules should be verified after installation
          () => {
            const p = module.init(core);
            if (p instanceof Promise) {
              return p;
            } else {
              return new Promise(
                (_, reject) => reject(
                  `Module: ${moduleName}, does not expose a Promise`
                  + ", and is therefore invalid"
                )
              );
            }
          },
          err => new Error(
            `Module: ${moduleName} threw Error on init: ${JSON.stringify(err)}`
          ),
        ),
        TE.match(
          err => err,
          u => u,
        ),
        el => el,
      )),
      el => el,
    ));
};
