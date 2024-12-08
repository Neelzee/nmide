import { AppConfig, AppOption, defaultConfig, GetOrElse, ModelOverwrite, THtml, TMap, TMsg } from "@nmide/js-utils";
import { InstallPlugins } from "./lib/InstallPlugins";
import { Init } from "./lib/Init";
import "@nmide/js-utils";
import { constant, pipe } from "fp-ts/lib/function";
import * as M from "fp-ts/Map";
import * as A from "fp-ts/Array";
import * as S from "fp-ts/string";
import { View } from "./lib/View";
import { Update } from "./lib/Update";
import { setTimeout } from "timers/promises";

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

  window.listen<TMsg>("msg", ({ payload: msg }) => {
    const plugins = M.toArray(S.Ord)(window.plugins);
    const prevState = window.state;
    Update(prevState, msg, plugins)
      .then(state => pipe(
        state,
        GetOrElse<[TMap, [string, TMap][]]>([[], []]),
        ([state, collisions]) => A.isEmpty(collisions)
          ? state
          : pipe(
            collisions,
            A.map(([pln, model]) => {
              window.log.error(`Collisions from: ${pln}, with model: `, model);
            }),
            constant(state),
          ),
      ))
      .then(newState => {
        window.state = ModelOverwrite(prevState, newState);
        window.emit("nmide://update").catch(err => window.log.error("emit update: ", err));
        return window.state;
      })
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
    .then(_ => setTimeout(250))
    .then(_ => M.toArray(S.Ord)(window.plugins))
    .then(plugins => Init(plugins))
    .then(init => pipe(
      init,
      GetOrElse<[TMap, [string, TMap][]]>([[], []]),
      ([state, collisions]) => A.isEmpty(collisions)
        ? state
        : pipe(
          collisions,
          A.map(([pln, model]) => {
            window.log.error(`Collisions from: ${pln}, with model: `, model);
          }),
          _ => state,
        ),
    ))
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

