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
      try {
        const module = event.detail;
        if (window.__nmideConfig__.modules.get(module.name) !== undefined) {
          return;
        }
        const core = await mkCore();
        await module.init(core);
        window.__nmideConfig__.modules.set(module.name, module)
      } catch (err) {
        window.__nmideConfig__
          .log
          .error(
            `Error initializing runtime module: ${err}, ${JSON.stringify(err)}`
          );
      }
    }
  );
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

// TODO: Add docs
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
