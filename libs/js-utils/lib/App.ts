import * as t from "io-ts";
import { TMap } from "./TMap";
import { TMsg } from "./TMsg";
import * as Decoder from "./Decoder";
import { THtml } from "./THtml";
import * as E from "fp-ts/lib/Either";
import * as A from "fp-ts/lib/Array";
import { pipe } from "fp-ts/lib/function";
import { fst } from "fp-ts/Tuple";

export interface AppOption {
  /**
     * List of clean up functions. The app calls the supplied function to clean
     * up the HTML that the plugin has rendered. If anything goes wrong calling
     * this function, it will be logged, with the supplied plugin name.
     */
  cleanup?: [string, (() => void)][];
  /**
     * List of path to all plugins, [pluginName, pluginPath].
     */
  pluginAssets?: [string, string][];
  /**
     * Parses the THtml using `parseHtml`, creating a corresponding HTMLElement,
     * which is set as a child of `window.root`.
     *
     * Returns undefined if the supplied THtml is of kind `Frag` and childless.
     */
  renderHtml?: (html: THtml) => HTMLElement | undefined;
  /**
     * Parses the THtml, creating a corresponding HTMLElement, returning it.
     *
     * Returns undefined if the supplied THtml is of kind `Frag` and childless.
     */
  parseHtml?: (html: THtml) => HTMLElement | undefined;
  /**
     * The HTMLElement that all THtml are children off. Corresponds to
     * document.body
     */
  root?: HTMLElement;
  /**
     * Client used to call the `backend`. In the IDE configuration, this
     * corresponds to the Tauri client, with a wrapper to decode the response.
     *
     * This is not necessary in production, as the types from the `backend` are
     * always known, but during development it's easier if miss-match in types
     * are caught before being supplied to plugins.
     */
  client?: NmideClient;

  /**
     * Logs different events in the application. The default is console.log, and
     * console.error, respectively.
     */
  log?: NmideLogger;

  /**
     * Used by the application to listen for the `Msg`-event.
     */
  listen?: <T>(event: string, handler: (x: Payload<T>) => void) => Promise<any>,
  /**
     * Used by the application to emit `Msg`-events.
     */
  emit?: <T>(event: string, payload?: T) => Promise<void>,

  /**
     * Used by the application to retrieve plugins. In the IDE configuration,
     * This is `$APPDATA/plugins`. This needs to be a promise, due to how
     * Tauri makes paths Os-agnostic.
     */
  getPluginPaths?: Promise<string[]>,

  /**
     * Installs plugins. For the IDE and Server, this is JS and CSS. The IDE can
     * additionally install `wb.html` plugins, which enables another window to
     * be created. This window is not managed by the core application, meaning
     * it exist outside the init-update-view-loop.
     */
  pluginInstallers?: ((path: string) => Promise<string | undefined>)[],

  /**
     * After every `init` and `update`, the new plugin state is coalced into a
     * single state. Before this, we check if any of the states have a
     * collision. A collision between two states occurs if they contain the same
     * field. The standard way to deal with a collision is to log it using
     * `window.log.error`, and drop it.
     */
  coalcePluginState: [(xs: E.Either<[[string, TMap][], string], [string, TMap]>[]) => [string, TMap][]],
}

export interface Payload<T> {
  payload: T
};

export type AppConfig = Required<AppOption>;

export type NmideLogger = {
  info: (msg: any, ...xs: any) => void,
  error: (msg: any, ...xs: any) => void,
}

export type NmideArgs = {
  "init": {
    args: undefined,
  },
  "view": {
    args: { tmodel: TMap },
  },
  "update": {
    args: { tmsg: TMsg, tmodel: TMap, },
  },
  "plugin_init": {
    args: { pluginName: string },
  },
  "plugin_view": {
    args: { pluginName: string, tmodel: TMap },
  },
  "plugin_update": {
    args: { pluginName: string, tmsg: TMsg, tmodel: TMap, },
  },
  "get_plugins": {
    args: undefined,
  }
}

const { DViewDecoder, DInitDecoder, DUpdateDecoder } = Decoder;

export const NmideDecoder = {
  "init": DInitDecoder,
  "view": DViewDecoder,
  "update": DUpdateDecoder,
  "plugin_init": Decoder.DMap,
  "plugin_view": Decoder.DHtml,
  "plugin_update": Decoder.DMap,
  "get_plugins": t.array(t.string),
}

export type NmideDecodedType<
  K extends keyof NmideArgs
  & keyof typeof NmideDecoder
> = t.TypeOf<typeof NmideDecoder[K]>


export type NmideClient = <
  K extends keyof NmideArgs & keyof typeof NmideDecoder,
  A extends NmideArgs[K]["args"]
>(
  cmd: K,
  args?: A,
) => Promise<E.Either<Error, NmideDecodedType<K>>>;


export const defaultConfig: AppConfig = {
  cleanup: [],
  pluginAssets: [],
  renderHtml: (_: THtml) => undefined,
  parseHtml: (_: THtml) => undefined,
  root: document.body,
  client: (_: any, __: any) => new Promise(() => { }),
  log: {
    info: console.log,
    error: console.error,
  },
  listen: (_: any, __: any) => new Promise(r => r(undefined)),
  emit: (_: any, __?: any) => new Promise(r => r()),
  getPluginPaths: new Promise(r => r([])),
  pluginInstallers: [(_: string) => new Promise(r => r(undefined))],
  coalcePluginState: [(xs: E.Either<[[string, TMap][], string], [string, TMap]>[]) => pipe(
    xs,
    A.map<E.Either<[[string, TMap][], string], [string, TMap]>, [string, TMap]>(
      E.getOrElse<[[string, TMap][], string], [string, TMap]>(([plgs, field]) => {
        const plugins = pipe(
          plgs,
          A.map(fst),
          A.reduce("", (a, b) => `${a}\n${b}`)
        );
        const state = A.map<[string, TMap], string>(
          ([p, s]) => `Plugin: ${p}, state: ${JSON.stringify(s)}`
        )(plgs);
        window.log.error(
          `Error on coalecing plugin state, on field: ${field}`
            + `. Affected plugins: ${plugins}`
            + `. State: ${state}`, plgs
        );
        return ["", []];
      })
    ),
    el => el,
  )],
};

export const getOpts = (opts?: AppOption): AppConfig => {
  if (opts === undefined) {
    return defaultConfig;
  }
  const partialConfig: Partial<AppConfig> = Object.fromEntries(
    Object.entries(opts).filter(([_, f]) => f !== undefined)
  );

  return { ...defaultConfig, ...partialConfig };
}
