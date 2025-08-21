import client, { listen } from "@nmide/js-client";
import { AppConfig } from "@nmide/js-utils";
import {
  defaultConfig,
  NMIDE_INITIALIZED,
} from "@nmide/js-core-std-lib";
import { info, debug, error } from "@tauri-apps/plugin-log";
import { tsRenderer as render } from "./tsRenderer.ts";
import { emit } from "@tauri-apps/api/event";
import * as E from "fp-ts/Either";

const App = {
  initialize: async (config: Partial<AppConfig> = {}) => {
    info("[frontend] Initialized");
    window.__nmideConfig__ = { ...defaultConfig, ...config, render }
    /**
     * NOTE: We overwrite the default `log`, so that we get the frontend
     * log-statements in the logs managed by Tauri aswell.
     *
     * @see [Tauri logging](https://v2.tauri.app/plugin/logging/)
     */
    // @ts-expect-error This is valid
    window.__nmideConfig__.log = { info, debug, error }
    document.dispatchEvent(new CustomEvent(NMIDE_INITIALIZED));
  },
  run: async () => {
    info!("[frontend] finished module installation");
    listen("nmide://render", ({ payload: ui }) => {
      window.__nmideConfig__.log.info(`[frontend] Rendering: ${JSON.stringify(ui)}`);
      window.__nmideConfig__.render(ui)
        .catch(err => window.__nmideConfig__.log.error(`Error on render: ${JSON.stringify(err)}`));
    })
      .then(err => {
        if (E.isLeft(err)) {
          const error = err.left;
          window.__nmideConfig__
            .log
            .error(`[frontend] listen nmide://render: ${JSON.stringify(error)}, ${error}`)
        }
      })
      .catch((err) => window.__nmideConfig__.log.error(`nmide://render: ${JSON.stringify(err)}`));

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
          .then(err => {
            if (E.isLeft(err)) {
              const error = err.left;
              window.__nmideConfig__
                .log
                .error(
                  `[frontend] client handler: ${JSON.stringify(error)}, ${error}`
                );
            }
          })
          .catch((err) => window.__nmideConfig__.log.error(`Handler: ${JSON.stringify(err)}`))
      }
      );
    })
      .then(err => {
        if (E.isLeft(err)) {
          const error = err.left;
          window.__nmideConfig__
            .log
            .error(
              `listen nmide://event: ${JSON.stringify(error)}, ${error}`
            );
        }
      })
      .catch((err) => window.__nmideConfig__.log.error(`nmide://event: ${JSON.stringify(err)}`));


    const promises = [];
    for (let i = 0; i < window.__nmideConfig__.runtimes.initializers.length; i++) {
      const init = window.__nmideConfig__.runtimes.initializers[i];
      promises.push(init());
    }
    await Promise.all(promises)
      .then(
        () => client("init")
          .then(err => {
            if (E.isLeft(err)) {
              const error = err.left;
              window.__nmideConfig__
                .log
                .error(
                  `[frontend] client init: ${JSON.stringify(error)}, ${error}`
                );
            }
          })
          .catch((err) => window.__nmideConfig__.log.error(`init: ${JSON.stringify(err)}`))
      );
  }
};

export default App;
