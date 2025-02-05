import * as t from "io-ts";
import { TMap } from "./TMap";
import { TMsg } from "./TMsg";
import * as Decoder from "./Decoder";
import * as E from "fp-ts/lib/Either";

export interface AppOption {
  /**
     * List of path to all plugins, [pluginName, pluginPath].
     */
  pluginAssets?: [string, string][];
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
  pluginAssets: [],
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
