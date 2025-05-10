import client, { listen } from "@nmide/js-client";
import { Module } from "@nmide/js-utils";
import { ideInstallModules } from "./lib/ideInstallModules.ts";
import { AppConfig, defaultConfig } from "@nmide/js-core-std-lib";
import {
    NMIDE_INITIALIZED,
    NMIDE_MODULES_INSTALLED_EVENT
} from "./nmideConstants.ts";
import { run } from "./main.ts";
import { tsInit, tsHandler } from "./tsRuntime.ts";
import { tsRenderer } from "./lib/tsRenderer.ts";
import { handlerRegistration } from "./lib/handlerRegistration.ts";
import { eventThrower } from "./lib/eventThrower.ts";

run({
    initialize: (config: Partial<AppConfig> = {}) => {
        const conf = { ...defaultConfig(tsRenderer, handlerRegistration, eventThrower), ...config };
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
        listen("nmide://render", ({ payload: ui }) => {
            window.__nmideConfig__.render(ui)
                .catch(err => window.__nmideConfig__.log.error("Error on render: ", err));
        }).catch((err) => window.__nmideConfig__.log.error("nmide://render", err));

        listen("nmide://event", ({ payload: event }) => {
            (async () => {
                const promises = [];
                for (let i = 0; i < window.__nmideConfig__.runtimes.handlers.length; i++) {
                    const init = window.__nmideConfig__.runtimes.handlers[i];
                    promises.push(init(typeof event === "object" ? { ...event } : event));
                }
                return await Promise.all(promises);
            })().then(cm => {
                const mods = cm.flat();
                client("handler", { event, mods })
                    .catch((err) => console.error("Handler: ", err))
            }
            );
        }).catch((err) => window.__nmideConfig__.log.error("nmide://event", err));

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
            .then(mods =>
                client("init", { mods })
                    .then(() => console.log("INIT"))
                    .catch((err) => console.error("Init: , with args: ", err, { mods }))
            );
    }
},
    {
        runtimes:
        {
            handlers: [tsHandler],
            initializers: [tsInit]
        },
    }
);