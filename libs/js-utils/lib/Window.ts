import { ModuleUnknown as Module } from "./Module";
import { THtml } from "./THtml";
import { NmideClient, NmideLogger, Payload } from "./App";
import { Core, Event } from "./Core";

declare global {
  interface Window {
    core: Core;
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
    state: () => Promise<object>;
    /**
     * The HTMLElement that all THtml are children off. Corresponds to
     * document.body
     */
    root: HTMLElement;
    /**
     * Map of all JSPs loaded.
     */
    modules: Map<string, Module>;
    moduleCount: number;
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
    // TODO: This returns an cleanup function, which is not used, as all Events
    // used by this `listen`, are designed to exist for the entirety of the
    // applications lifetime, there is no need for this cleanup function to
    // exist
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
    events: Event[];
    eventHandlers: Map<string, string[]>;
  }
}
