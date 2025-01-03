import { AsyncNmluginUnknown } from "./Nmlugin";
import { ModuleUnknown as Module } from "./Module";
import { THtml } from "./THtml";
import { TMap } from "./TMap";
import { NmideClient, NmideLogger, Payload } from "./App";
import { Either } from "fp-ts/lib/Either";

declare global {
  interface Window {
    /**
     * List of path to all plugins, [pluginName, pluginPath].
     */
    pluginAssets: [string, string][];
    /**
     * List of clean up functions. The app calls the supplied function to clean
     * up the HTML that the plugin has rendered. If anything goes wrong calling
     * this function, it will be logged, with the supplied plugin name.
     */
    cleanup: [string, (() => void)][];
    /**
     * Parses the THtml using `parseHtml`, creating a corresponding HTMLElement,
     * which is set as a child of `window.root`.
     *
     * Returns undefined if the supplied THtml is of kind `Frag` and childless.
     */
    renderHtml: (html: THtml) => HTMLElement | undefined;
    /**
     * Parses the THtml, creating a corresponding HTMLElement, returning it.
     *
     * Returns undefined if the supplied THtml is of kind `Frag` and childless.
     */
    parseHtml: (html: THtml) => HTMLElement | undefined;
    /**
     * State of the application
     */
    state: TMap;
    /**
     * The HTMLElement that all THtml are children off. Corresponds to
     * document.body
     */
    root: HTMLElement;
    /**
     * Map of all JSPs loaded.
     */
    plugins: Map<string, Module>;
    moduleCount: number;
    /**
     * TBD
     */
    async_plugins: Map<string, AsyncNmluginUnknown>;
    /**
     * Client used to call the `backend`. In the IDE configuration, this
     * corresponds to the Tauri client, with a wrapper to decode the response.
     *
     * This is not necessary in production, as the types from the `backend` are
     * always known, but during development it's easier if miss-match in types
     * are caught before being supplied to plugins.
     */
    client: NmideClient;
    /**
     * Logs different events in the application. The default is console.log, and
     * console.error, respectively.
     */
    log: NmideLogger;
    /**
     * Used by the application to listen for the `Msg`-event.
     */
    listen: <T>(event: string, handler: (x: Payload<T>) => void) => Promise<any>,
    /**
     * Used by the application to emit `Msg`-events.
     */
    emit: <T>(event: string, payload?: T) => Promise<void>,
    /**
     * Used by the application to retrieve plugins. In the IDE configuration,
     * This is `$APPDATA/plugins`. This needs to be a promise, due to how
     * Tauri makes paths Os-agnostic.
     */
    getPluginPaths: Promise<string[]>,
    /**
     * Installs plugins. For the IDE and Server, this is JS and CSS. The IDE can
     * additionally install `wb.html` plugins, which enables another window to
     * be created. This window is not managed by the core application, meaning
     * it exist outside the init-update-view-loop.
     */
    pluginInstallers: ((path: string) => Promise<string | undefined>)[],
    /**
     * After every `init` and `update`, the new plugin state is coalced into a
     * single state. Before this, we check if any of the states have a
     * collision. A collision between two states occurs if they contain the same
     * field. The standard way to deal with a collision is to log it using
     * `window.log.error`, and drop it.
     */
    coalcePluginState: [(xs: Either<[[string, TMap][], string], [string, TMap]>[]) => [string, TMap][]],
  }
}
