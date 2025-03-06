import "@nmide/js-utils";
import { pipe } from "fp-ts/lib/function";
import * as A from "fp-ts/Array";
import * as TE from "fp-ts/TaskEither";
import * as T from "fp-ts/Task";

type ModuleInstaller = ((path: string) => Promise<string | undefined>);

// TODO: Add docs
// NOTE: Installation has side-effects!
/**
* Since all of these promises come from the _outside_, they cannot be
* trusted. `Promise.all` cannot be used, because that would mean all
* successfully installed modules are ignored, if _any_ module-installation
* errors. `Promise.allSettled` cannot be used either, because any exception
* thrown by a module would be ignored, which is not helpefull behaviour. To
* solve this, all promises must be wrapped in an `Task` from `fp-ts`, which is
* a promise that **never** fails. So, for all promises, we try-catch, and map
* the result to an `Either`.
*/
export const InstallPlugins: T.Task<string[]> = pipe(
  T.flatten(() => window.getPluginPaths
    .then(modulePaths => pipe(
      window.pluginInstallers,
      A.flatMap<ModuleInstaller, T.Task<string | undefined>>(
        (mi, i) => {
          window.log.info("Running installer: ", i);
          return moduleInstallerWrapper(modulePaths)(mi);
        }
      ),
      //A.append<T.Task<string | undefined>>(resolveOnInstallation),
      // TODO: Remove
      A.append<T.Task<string | undefined>>(() => new Promise(resolve => {
        const checkInterval = setInterval(() => {
          clearInterval(checkInterval);
          resolve(undefined);
        }, 450);
      })),
      T.sequenceArray,
    ))
  ),
  T.map<readonly (string | undefined)[], string[]>(
    xs => xs.filter(x => x !== undefined)
  ),
);

// HACK: _clever_ way to solve the issue with module loading. Since a module is
// _installed_ by creating a script-tag, and letting the webview parse it, the
// `Core` has to wait until a module is _loaded_, which is when the module is
// inserted into `window.plugins`, once this happens, the `window.moduleCount`
// is decremented. It is incremented for each module _installed_. So once it is
// 0, all modules have been _installed_ and _loaded_, so the Core is ready to
// start. Noted as a **hack** because I am unsure if this is the best way to
// solve this issue.
const resolveOnInstallation: T.Task<string | undefined> = () =>
  new Promise(resolve => {
    const c = { count: 0 };
    const checkInterval = setInterval(() => {
      if (window.moduleCount === 0) {
        clearInterval(checkInterval);
        resolve(undefined);
      } else {
        c.count += 1;
        window.log.info(`Resolve Count: ${c.count}, Module Count: ${window.moduleCount}`);
      }
    }, 50);
  })

// TODO: Add docs
const moduleInstallerWrapper = (modules: string[]) =>
  (f: ModuleInstaller): T.Task<string | undefined>[] => pipe(
    modules,
    A.map(m => pipe(
      TE.tryCatch(
        () => f(m).then(v => {
          window.moduleCount++;
          return v;
        }),
        (reason) => new Error(
          `Error on module installation: ${reason}, from module: ${m}`
        )
      ),
    )),
    A.map(TE.fold(
      err => {
        window.log.error(`Module Installation:`, err);
        return T.of(undefined);
      },
      T.of
    )),
  );
