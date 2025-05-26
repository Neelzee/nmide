/**
 * Runtime module installer
 *
 * A JavaScript module is just a script tag, with a JavaScript source. Normally,
 * modules are installed during compile time. Installation, in this context,
 * simply means that the modules are added to the
 * `window.__nmideConfig__.modules` map, which is handled by the `installModule`
 * function, maintained by @nmide/js-utils (in lib/Module.ts). This function,
 * waits until the event: `NMIDE_INITIALIZED = "nmide://initialized"`, to be
 * dispatched from the `document`. But this happens once, meaning if a module is
 * added to the `document` after this, it will never be installed. To correct
 * for this, another eventListener was added, which listens for:
 * `NMIDE_RT_MODULE_INSTALLED_EVENT = "nmide://rtModuleInstalled"`, and then,
 * if the module is not already installed, it dispatches an event:
 * `NMIDE_RT_MODULE_PUSHED_EVENT = "nmide://rtModulePushed"`, along with the
 * Module to be installed. We can then ensure the Module gets `init`-ilized, and
 * added to the module map.
 *
 * For more information about compile time modules, see `../index.ts`.
 * @see [fp-ts](https://gcanti.github.io/fp-ts/)
 */

import { pipe } from "fp-ts/lib/function";
import * as A from "fp-ts/Array";
import * as RA from "fp-ts/ReadonlyArray";
import * as TE from "fp-ts/TaskEither";
import * as T from "fp-ts/Task";
import * as S from "fp-ts/string";
import { DirEntry, readDir } from "@tauri-apps/plugin-fs";
import { appDataDir, join } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/core";
import { rtJsmInstaller } from "./rtJsmInstaller.ts";
import { mkCore, NMIDE_RT_MODULE_PUSHED_EVENT } from "@nmide/js-core-std-lib";
import { Module } from "@nmide/js-utils";

type ModuleInstaller = ((path: string) => Promise<string | undefined>);

document.addEventListener("DOMContentLoaded", async () => {
  // @ts-expect-error This is a valid eventListener
  document.addEventListener(
    NMIDE_RT_MODULE_PUSHED_EVENT,
    async (event: CustomEvent<Module>) => {
      /*
       * Wrapped in a try catch, because we can't be sure the dispatched event
       * is what we expect it to be. This could be handled better, by using the
       * decoding stuff we have.
       */
      try {
        const module = event.detail;
        if (window.__nmideConfig__.modules.get(module.name) !== undefined) {
          return;
        }
        const core = await mkCore();
        await module.init(core);
        window.__nmideConfig__.modules.set(module.name, module)
      } catch (err) {
        // NOTE: We don't add the module to the modules map, if it errors during
        // initialization
        window.__nmideConfig__
          .log
          .error(
            `Error initializing runtime module: ${err}, ${JSON.stringify(err)}`
          );
      }
    }
  );
  // Corresponds to `~/.local/share/no.nilsmf.uib/` on linux (debian)
  const pluginDir = await appDataDir()
    .then(p => join(p, "modules"));
  const paths = await readDir(pluginDir)
    .then(paths => {
      return paths;
    })
    .then(dirs => pipe(
      dirs,
      A.filter(d => d.isFile),
    ))
    .then(A.map<DirEntry, Promise<string>>(p => join(pluginDir, p.name)))
    .then(A.map<Promise<string>, Promise<string>>(p => p.then(convertFileSrc)))
    .then(paths => Promise.all(paths))
    .then(A.sort(S.Ord));
  window.__nmideConfig__.log
    .info(`[frontend] module paths: ${JSON.stringify(paths)}`);
  const installers = [rtJsmInstaller];
  const modulesPromise = pipe(
    installers,
    A.flatMap(moduleInstallerWrapper(paths)),
    T.sequenceArray,
    T.map(RA.toArray),
    T.map<(string | undefined)[], string[]>(A.filter(a => a !== undefined)),
  );
  const modules = await modulesPromise();
  window.__nmideConfig__.log
    .info(`[frontend] Runtime modules: ${JSON.stringify(modules)}`);
});

/**
 * Wraps module installers in a try-catch, ensuring if the module installer for
 * some reason throws an error, it does not crash the entire application.
 */
const moduleInstallerWrapper = (modules: string[]) =>
  (f: ModuleInstaller): T.Task<string | undefined>[] => pipe(
    modules,
    A.map(m => pipe(
      TE.tryCatch(
        () => f(m),
        (err) => new Error(
          `Error on module installation: ${err}, from module: ${m}, ${JSON.stringify(err)}`
        )
      ),
    )),
    A.map(TE.fold(
      err => {
        window.__nmideConfig__.log.error(`Module Installation: ${err}, ${JSON.stringify(err)}`, err);
        return T.of(undefined);
      },
      T.of
    )),
  );
