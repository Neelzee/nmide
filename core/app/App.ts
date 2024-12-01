import { GetOrElse, ModelOverwrite, THtml, TMap, TMsg } from "@nmide/js-utils";
import { InstallPlugins } from "./lib/InstallPlugins";
import { Init } from "./lib/Init";
import "@nmide/js-utils";
import { constant, pipe } from "fp-ts/lib/function";
import * as M from "fp-ts/Map";
import * as A from "fp-ts/Array";
import * as S from "fp-ts/string";
import { View } from "./lib/View";
import { renderHtml } from "./lib/renderHtml";
import { Update } from "./lib/Update";

export const App = (): void => {
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

      .then(newState => {
        window.cleanup.forEach(([pln, clean]) => {
          try {
            clean();
          } catch (err) {
            window.log.error(`Error on Cleanup from plugin: ${pln}, `, err);
          }
        });
        return newState;
      })
      .then(state => View(plugins, state))
      .then(htmls => pipe(
        htmls,
        A.map<[string, THtml], [string, (() => void)]>(
          ([x, y]) => [x, (() => {
            const elem = window.renderHtml(y);
            if (elem === undefined) return
            window.root.removeChild(elem);
          })]
        ),
      )).then(cleanup => {
        window.cleanup = cleanup;
      })
      .then(_ => window.emit("nmide://view").catch(err => window.log.error("emit view: ", err)));
  });

  InstallPlugins()
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
      A.map<[string, THtml], [string, (() => void)]>(
        ([x, y]) => [x, (() => {
          const elem = renderHtml(y);
          if (elem === undefined) return
          window.root.removeChild(elem);
        })]
      ),
    ))
    .then(cleanup => {
      window.cleanup = window.cleanup.concat(cleanup);
    })
    .then(_ => {
      window.emit("nmide://view").catch(err => window.log.error("emit view: ", err));
    });
};

