import { pipe } from "fp-ts/lib/function";
import * as A from "fp-ts/Array";
import * as TE from "fp-ts/TaskEither";
import * as T from "fp-ts/Task";
import * as S from "fp-ts/string";
import { DirEntry, readDir } from "@tauri-apps/plugin-fs";
import { appDataDir, join } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/core";
import { jsmInstaller } from "./jsmInstaller.ts";
import { cssInstaller } from "./cssInstaller.ts";
import { htmlInstaller } from "./htmlInstaller.ts";

// TODO: Add docs
export const getPaths = async (): Promise<string[]> => {
  const pluginDir = await appDataDir()
    .then(p => join(p, "modules"));
  return readDir(pluginDir)
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
    .then(A.sort(S.Ord))
    .then(p => {
      window.__nmideConfig__.log.info(`[frontend] module paths: ${JSON.stringify(p)}`);
      return p;
    });
};

type ModuleInstaller = ((path: string) => Promise<string | undefined>);

// TODO: Add docs
// NOTE: Installation has side-effects!
/**
* Since all of these promises come from the _outside_, they cannot be
* trusted. `Promise.all` cannot be used, because that would mean all
* successfully installed modules are ignored, if _any_ module-installation
* errors. `Promise.allSettled` cannot be used either, because any exception
* thrown by a module would be ignored, which is not helpful behaviour. To
* solve this, all promises must be wrapped in an `Task` from `fp-ts`, which is
* a promise that **never** fails. So, for all promises, we try-catch, and map
* the result to an `Either`.
*/
export const ideInstallModules: T.Task<string[]> = pipe(
  T.flatten(() => getPaths()
    .then(modulePaths => pipe(
      [jsmInstaller, cssInstaller, htmlInstaller],
      A.flatMap<ModuleInstaller, T.Task<string | undefined>>(
        (mi) => {
          return moduleInstallerWrapper(modulePaths)(mi);
        }
      ),
      T.sequenceArray,
    ))
  ),
  T.map<readonly (string | undefined)[], string[]>(
    xs => xs.filter(x => x !== undefined)
  ),
);


// TODO: Add docs
const moduleInstallerWrapper = (modules: string[]) =>
  (f: ModuleInstaller): T.Task<string | undefined>[] => pipe(
    modules,
    A.map(m => pipe(
      TE.tryCatch(
        () => f(m).then(v => {
          if (v !== undefined) window.__nmideConfig__.moduleCount++;
          return v;
        }),
        (reason) => new Error(
          `Error on module installation: ${reason}, from module: ${m}`
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
