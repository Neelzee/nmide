import {
  AppConfig,
  AppOption,
  defaultConfig,
  tObj,
  TValue,
} from "@nmide/js-utils";
import { InstallPlugins } from "./lib/InstallPlugins";
import "@nmide/js-utils";
import { pipe } from "fp-ts/lib/function";
import * as M from "fp-ts/Map";
import { Monoid } from "fp-ts/Monoid";
import { foldMap } from "fp-ts/ReadonlyArray";
import * as A from "fp-ts/Array";
import * as S from "fp-ts/string";
import * as E from "fp-ts/Either";
import * as TE from "fp-ts/TaskEither";
import * as T from "fp-ts/Task";
import { ModuleUnknown as Module } from "@nmide/js-utils/lib/Module";
import { Core } from "@nmide/js-utils/lib/Core";
import { DCore } from "@nmide/js-utils/lib/Decoder";
import { evalTree } from "@nmide/js-utils/lib/evalTree";
import { toNode, Node } from "@nmide/js-utils/lib/tree";
import { Errors } from "io-ts";

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

  InstallPlugins()
    .then(() => M.toArray(S.Ord)(window.plugins))
    .then(modules => pipe(
      modules,
      A.map(([moduleName, module]) => pipe(
        TE.tryCatch<Error, unknown>(
          // HACK: Modules should be verified after installation. Or maybe its
          // okay to this here, I dont really know.
          () => {
            if (typeof module.init === "function") {
              return module.init(core);
            } else {
              // TODO: Is there a better way to verify a third-party function?
              return new Promise(
                (_, reject) => reject(
                  `Module: ${moduleName}, does not expose a function`
                  + ", and is therefore invalid"
                )
              );
            }
          },
          err => new Error(
            `Module: ${moduleName} threw Error on init: ${JSON.stringify(err)}`
          ),
        ),
        TE.match<Error, E.Either<Error, Core>, unknown>(
          E.left,
          u => pipe(
            u,
            DCore.decode,
            E.mapLeft<Errors, Error>(
              err => new Error(`Error on decode: ${JSON.stringify(err)}`)
            ),
          )
        )
      )),
      T.sequenceArray,
      T.map(xs => pipe(
        xs,
        foldMap(CoreMonoid(core))(E.getOrElse(err => {
          window.log.error("Error on core folding, post installation:", err);
          return core;
        }))
      )),
      task => task(),
    )) // TODO: Pass the core to the runtime handling event triggering
    .then(core => core);
};

const CoreMonoid = (empty: Core): Monoid<Core> => {
  return {
    empty,
    concat: CoreSemiGroup,
  }
};

// TODO: This seems overkill/stupid/slow, but if it is _just_ milliseconds slow,
// then it is okay
const CoreSemiGroup = (a: Core, b: Core): Core => {
  return {
    ui: evalTree(
      { root: toNode(a.ui) },
      A.concat(a.uiModifications)(b.uiModifications)
    ).root,
    uiModifications: [],
    state: pipe(
      evalTree(
        {
          root: {
            id: "root",
            kids: pipe(
              a.state,
              A.map(([field, value]) => {
                return { id: field, kids: [] as Node<TValue>[], ...value };
              })
            ),
            ...tObj([])
          }
        },
        A.concat(a.stateModifications)(b.stateModifications)
      ),
      tree => tree.root,
      node => node.kids,
      A.map(({ id, kids: _, ...value }) => [id, value])
    ),
    stateModifications: [],
    events: pipe(
      evalTree(
        {
          root: {
            id: "root",
            kids: pipe(
              a.events,
              A.map(evt => {
                return toNode(evt);
              })
            ),
            event: "",
            module: "",
          }
        },
        A.concat(a.eventModifications)(b.eventModifications)
      ),
      tree => tree.root,
      node => node.kids,
      A.map(({ id: _, kids: __, ...evt }) => evt)
    ),
    eventModifications: []
  };
}
