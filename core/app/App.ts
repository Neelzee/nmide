import { GetOrElse, ModelOverwrite, THtml, TMap, TMsg } from "@nmide/js-utils";
import { listen } from "@tauri-apps/api/event";
import { InstallPlugins, LoadPlugins } from "./lib/InstallPlugins";
import { Init } from "./lib/Init";
import "@nmide/js-utils";
import { constant, pipe } from "fp-ts/lib/function";
import * as M from "fp-ts/Map";
import * as A from "fp-ts/Array";
import * as S from "fp-ts/string";
import { View } from "./lib/View";
import { renderHtml } from "./lib/renderHtml";
import { Update } from "./lib/Update";
import { NmDebugLogMsg } from "@nmide/js-utils/lib/Debug";

export const App = (): void => {
  listen<TMsg>("msg", ({ payload: msg }) => {
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
            NmDebugLogMsg("Collisions"),
            A.map(([pln, model]) => {
              console.debug(`Collisions from: ${pln}, with model: `, model);
            }),
            constant(state),
          ),
      ))
      .then(newState => {
        window.state = ModelOverwrite(prevState, newState);
        return window.state;
      })
      .then(state => View(plugins, state))
      .then(htmls => pipe(
        htmls,
        A.map<[string, THtml], [string, HTMLElement]>(([x, y]) => [x, renderHtml(y)]),
      ))
      .then(newCleanup => {
        window.cleanup.forEach(([_, e]) => window.root.removeChild(e));
        window.cleanup = newCleanup;
      });
  });

  InstallPlugins()
    .then(_ => LoadPlugins())
    .then(plugins => Init(plugins))
    .then(init => pipe(
      init,
      GetOrElse<[TMap, [string, TMap][]]>([[], []]),
      ([state, collisions]) => A.isEmpty(collisions)
        ? state
        : pipe(
          collisions,
          A.map(([pln, model]) => {
            console.debug(`Collisions from: ${pln}, with model: `, model);
          }),
          _ => state,
        ),
    ))
    .then(state => {
      window.state = state;
      return state;
    })
    .then(tmodel => View(M.toArray(S.Ord)(window.plugins), tmodel))
    .then(htmls => pipe(
      htmls,
      A.map<[string, THtml], [string, HTMLElement]>(([x, y]) => [x, renderHtml(y)]),
    ))
    .then(cleanup => {
      window.cleanup = window.cleanup.concat(cleanup);
    });
};

