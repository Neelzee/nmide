import client, { listen } from "@nmide/js-client";
import { AppConfig, Module } from "@nmide/js-utils";
import { ideInstallModules } from "./ideInstallModules.ts";
import { defaultConfig } from "@nmide/js-core-std-lib";
import {
  NMIDE_INITIALIZED,
  NMIDE_MODULES_INSTALLED_EVENT
} from "./nmideConstants.ts";
import { info, debug, error } from "@tauri-apps/plugin-log";
import { tsRenderer as render } from "./tsRenderer.ts";
import { emit } from "@tauri-apps/api/event";

const App = {
  initialize: (config: Partial<AppConfig> = {}) => {
    info("[frontend] Initialized");
    const conf = { ...defaultConfig, ...config, log: { info, debug, error }, render };
    // @ts-expect-error This is okay
    window.__nmideConfig__ = {}
    // @ts-expect-error This is okay
    Object.keys(conf).forEach(key => { window.__nmideConfig__[key] = conf[key] });

    /* HACK: This makes it so that we get an accurate count of how many modules
     * are installed, since they install asynchronously, we have to "wait" until
     * the count is 0
    */
    const originalSetter =
      window.__nmideConfig__.modules
        .set.bind(window.__nmideConfig__.modules);
    window.__nmideConfig__.modules.set = (key: string, val: Module) => {
      const res = originalSetter(key, val);
      window.__nmideConfig__.moduleCount--;
      if (window.__nmideConfig__.moduleCount === 0 && !window.__nmideConfig__.installed) {
        window.__nmideConfig__.installed = true;
        document.dispatchEvent(
          new CustomEvent(NMIDE_MODULES_INSTALLED_EVENT)
        );
      }
      return res;
    };
    document.dispatchEvent(new CustomEvent(NMIDE_INITIALIZED));
  },
  installModules: () => {
    info!("[frontend] installing modules");
    ideInstallModules().then(() => {
      if (window.__nmideConfig__.moduleCount === 0 && !window.__nmideConfig__.installed) {
        window.__nmideConfig__.installed = true;
        document.dispatchEvent(
          new CustomEvent(NMIDE_MODULES_INSTALLED_EVENT)
        );
      }
    }).catch(err => {
      window.__nmideConfig__.log.error("Error on installation: ", err)
      if (window.__nmideConfig__.moduleCount === 0 && !window.__nmideConfig__.installed) {
        window.__nmideConfig__.installed = true;
        document.dispatchEvent(
          new CustomEvent(NMIDE_MODULES_INSTALLED_EVENT)
        );
      }
    });
  },
  run: () => {
    info!("[frontend] finished module installation");
    listen("nmide://render", ({ payload: ui }) => {
      window.__nmideConfig__.log.info(`[frontend] Rendering: ${JSON.stringify(ui)}`);
      window.__nmideConfig__.render(ui)
        .catch(err => window.__nmideConfig__.log.error(`Error on render: ${JSON.stringify(err)}`));
    }).catch((err) => window.__nmideConfig__.log.error(`nmide://render: ${JSON.stringify(err)}`));

    (async () => {
      window.__nmideConfig__.log.info("[frontend] sending events");
      window.__nmideConfig__.events.forEach(event => {
        emit("nmide://event", { event })
          .catch(err =>
            window.__nmideConfig__
              .log
              .error(
                `[frontend] Error from Event: ${JSON.stringify(event)}, Error: ${err}, ${JSON.stringify(err)}`
              )
          );
      });
      window.__nmideConfig__.events = [];
    })();

    listen("nmide://event", ({ payload: event }) => {
      window.__nmideConfig__.log.info(`[frontend] Event: ${JSON.stringify(event)}`);
      (async () => {
        const promises = [];
        for (let i = 0; i < window.__nmideConfig__.runtimes.handlers.length; i++) {
          const handler = window.__nmideConfig__.runtimes.handlers[i];
          promises.push(handler(event));
        }
        return await Promise.all(promises);
      })().then(() => {
        client("handler", { event })
          .catch((err) => window.__nmideConfig__.log.error(`Handler: ${JSON.stringify(err)}`))
      }
      );
    }).catch((err) => window.__nmideConfig__.log.error(`nmide://event: ${JSON.stringify(err)}`));

    new Promise(resolve => {
      const checkInterval = setInterval(() => {
        clearInterval(checkInterval);
        resolve(undefined);
      }, 450);
    }).then(async () => {
      const promises = [];
      for (let i = 0; i < window.__nmideConfig__.runtimes.initializers.length; i++) {
        const init = window.__nmideConfig__.runtimes.initializers[i];
        promises.push(init());
      }
      const mods = await Promise.all(promises);
      return mods.flat()
    })
      .then(() =>
        client("init")
          .catch((err) => window.__nmideConfig__.log.error(`init: ${JSON.stringify(err)}`))
      );
  }
};

export default App;
