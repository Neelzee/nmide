import {
  AppConfig,
  AppOption,
  defaultConfig,
  ModelOverwrite,
  THtml,
  TMsg
} from "@nmide/js-utils";
import { InstallPlugins } from "./lib/InstallPlugins";
import { Init } from "./lib/Init";
import "@nmide/js-utils";
import { pipe } from "fp-ts/lib/function";
import * as M from "fp-ts/Map";
import * as A from "fp-ts/Array";
import * as S from "fp-ts/string";
import { View } from "./lib/View";
import { Update } from "./lib/Update";
import { setTimeout } from "timers/promises";

// TODO: Add docs
export const App = (opts?: AppOption): void => {

  if (opts === undefined) {
    opts = defaultConfig;
  }

  const partialConfig: Partial<AppConfig> = Object.fromEntries(
    Object.entries(opts).filter(([_, v]) => v !== undefined)
  );

  const config: AppConfig = { ...defaultConfig, ...partialConfig };

  window.plugins = new Map();
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

  window.listen<TMsg>("msg", ({ payload: msg }) => {
    const plugins = M.toArray(S.Ord)(window.plugins);
    const prevState = window.state;
    Update(msg, plugins, prevState)
      .then(newState => {
        window.state = ModelOverwrite(prevState, newState);
        window.emit("nmide://update").catch(err => window.log.error("emit update: ", err));
        return window.state;
      })
      // HACK: Try-Catch on cleanup, because it is exsposed to other plugins.
      // But should we care? If a plugin introduces a new clean-up, is this
      // pure? Maybe. Should look into this. If this is the case, then we should
      // handle this better, i.e outside `App.ts`, and only keep the _happy_
      // path here.
      .then(state => {
        window.cleanup.forEach(([pln, clean]) => {
          try {
            clean();
          } catch (err) {
            window.log.error(`Error on Cleanup from plugin: ${pln}, `, err);
          }
        });
        window.cleanup = []
        return View(plugins, state);
      })
      .then(htmls => pipe(
        htmls,
        A.map<[string, THtml], [string, Element | undefined]>(
          ([x, y]) => [x, window.renderHtml(y)]
        ),
        A.filter((x): x is [string, Element] => x[1] !== undefined),
        A.map<[string, Element], [string, (() => void)]>(([x, y]) => [x, () => window.root.removeChild(y)]),
      ))
      .then(cleanup => {
        window.cleanup = window.cleanup.concat(cleanup);
      })
      .then(_ => window.emit("nmide://view").catch(err => window.log.error("emit view: ", err)));
  });

  InstallPlugins()
    // HACK: This timeout is here, because a plugin is installed by adding a
    // script-tag, and after this document has finished loading, it will be
    // added to `window.plugins`, currently there is no way to guarantee that
    // a plugin has finished _installing_. With the current test-plugins, a
    // timeout of `250 ms`, has a 100% success rate, but if a Plugin is larger
    // or does something that takes longer than `250 ms`-post loading, it will
    // result in the plugin not being loaded, and therefore not being loaded
    // when the `init`-state happens.
    .then(_ => setTimeout(250))
    .then(_ => M.toArray(S.Ord)(window.plugins))
    .then(plugins => Init(plugins))
    .then(state => {
      window.state = state;
      window.emit("nmide://init").catch(err => window.log.error("emit init: ", err));
      return state;
    })
    .then(tmodel => View(M.toArray(S.Ord)(window.plugins), tmodel))
    .then(htmls => pipe(
      htmls,
      A.map<[string, THtml], [string, Element | undefined]>(
        ([x, y]) => [x, window.renderHtml(y)]
      ),
      A.filter((x): x is [string, Element] => x[1] !== undefined),
      A.map<[string, Element], [string, (() => void)]>(([x, y]) => [x, () => window.root.removeChild(y)]),
    ))
    .then(cleanup => {
      window.cleanup = window.cleanup.concat(cleanup);
    })
    .then(_ => {
      window.emit("nmide://view").catch(err => window.log.error("emit view: ", err));
    });
};
