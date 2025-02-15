import {
  AppConfig,
  AppOption,
  defaultConfig,
} from "@nmide/js-utils";
import { InstallPlugins } from "./lib/InstallPlugins";
import "@nmide/js-utils";
import { pipe } from "fp-ts/lib/function";
import * as M from "fp-ts/Map";
import { reduce, map } from "fp-ts/ReadonlyArray";
import * as A from "fp-ts/Array";
import * as S from "fp-ts/string";
import * as E from "fp-ts/Either";
import * as TE from "fp-ts/TaskEither";
import * as T from "fp-ts/Task";
import { ModuleUnknown as Module } from "@nmide/js-utils/lib/Module";
import { Core, CoreModification, EventHandler, } from "@nmide/js-utils/lib/Core";
import { DCoreModification } from "@nmide/js-utils/lib/Decoder";
import { evalTree } from "@nmide/js-utils/lib/evalTree";
import { runtime } from "./runtime";

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

  InstallPlugins()
    .then(() => M.toArray(S.Ord)(window.plugins))
    .then(modules => pipe(
      modules,
      A.map(([moduleName, module]) => pipe(
        TE.tryCatch<Error, [string, unknown]>(
          async () => typeof module.init === "function"
            ? [moduleName, await module.init(core)]
            : new Promise(
              (_, reject) => reject(
                `Module: ${moduleName}, does not expose a function`
                + ", and is therefore invalid"
              )
            ),
          err => new Error(
            `Module: ${moduleName} threw Error on init: ${JSON.stringify(err)}`
          ),
        ),
        // NOTE: Any output from a Module is unknown, so this step is just to
        // verify that any given module returns the expected argument
        TE.match<Error, E.Either<Error, CoreModification>, [string, unknown]>(
          E.left,
          u => pipe(
            u,
            DCoreModification.decode,
            E.mapLeft(
              err => new Error(`Error on decode: ${JSON.stringify(err)}`)
            ),
          )
        )
      )),
      T.sequenceArray,
      T.map(xs => pipe(
        xs,
        map(E.getOrElse<Error, CoreModification>(err => {
          window.log.error("Error on core folding, post installation:", err);
          return {
            uiModifications: [],
            stateModifications: [],
            eventModifications: [],
            newEventHandlers: [],
          };
        })),
        reduce(core, coreEvaluation)
      )),
      runtime,
    ))
};

export const coreEvaluation = (
  core: Core,
  coreModifications: CoreModification
): Core => {
  return {
    ...core,
    ui: pipe(
      evalTree({ root: core.ui }, coreModifications.uiModifications),
      tree => tree.root
    ),
    state: pipe(
      evalTree({ root: core.state }, coreModifications.stateModifications),
      tree => tree.root
    ),
    events: pipe(
      evalTree({ root: core.events }, coreModifications.eventModifications),
      tree => tree.root
    ),
    eventHandlers: ((): Map<string, EventHandler[]> => {
      const map = core.eventHandlers;
      return pipe(
        coreModifications.newEventHandlers,
        // NOTE: Closest I know how to verify that the given handler is valid.
        // Atleast during runtime.
        A.filter(([_, { handler }]) => handler.length >= 1),
        A.map(([e, h]) => {
          let handlers = map.get(e);
          if (handlers === undefined) {
            handlers = [];
          }
          // HACK: Is this true? Don't know, but it needs to be true to fit
          // with the typechecking
          handlers.push(h as EventHandler)
          map.set(e, handlers);
        }),
        _ => map
      );
    })(),
  };
}
