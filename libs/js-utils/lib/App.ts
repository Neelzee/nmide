import * as t from "io-ts";
import { TMap } from "./TMap";
import { TMsg } from "./TMsg";
import * as Decoder from "./Decoder";
import { THtml } from "./THtml";
import * as E from "fp-ts/lib/Either";
import * as A from "fp-ts/lib/Array";
import { pipe } from "fp-ts/lib/function";
import { fst } from "fp-ts/Tuple";
import { ModelOverwrite } from "./Utils";

export interface AppOption {
  cleanup?: [string, (() => void)][];
  pluginAssets?: [string, string][];
  renderHtml?: (html: THtml) => HTMLElement | undefined;
  parseHtml?: (html: THtml) => HTMLElement | undefined;
  root?: HTMLElement;
  client?: NmideClient;
  log?: NmideLogger;
  listen?: <T>(event: string, handler: (x: Payload<T>) => void) => Promise<any>,
  emit?: <T>(event: string, payload?: T) => Promise<void>,
  getPluginPaths?: Promise<string[]>,
  pluginInstallers?: ((path: string) => Promise<string | undefined>)[],
  filterPluginState: (xs: [string, E.Either<Error, TMap>][]) => [string, TMap][],
  coalcePluginState: (xs: E.Either<[string, [string, TMap][]], TMap>[]) => TMap,
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
  filterPluginState: (xs: [string, E.Either<Error, TMap>][]): [string, TMap][] => pipe(
    xs,
    A.map<[string, E.Either<Error, TMap>], [string, TMap]>(
      ([pln, e]) => [pln, E.getOrElse<Error, TMap>(err => {
        window.log.error(`Error on plugin state, from plugin: ${pln}`, err);
        return [];
      })(e)]
    ),
  ),
  coalcePluginState: (xs: E.Either<[string, [string, TMap][]], TMap>[]) => pipe(
    xs,
    A.map<E.Either<[string, [string, TMap][]], TMap>, TMap>(
      E.getOrElse<[string, [string, TMap][]], TMap>(([field, ys]) => {
        const plugins = pipe(
          ys,
          A.map(fst),
          A.reduce("", (a, b) => `${a}\n${b}`),
        );
        const state = A.map<[string, TMap], string>(
          ([p, s]) => `Plugin: ${p}, state: ${JSON.stringify(s)}`
        )(ys);
        window.log.error(
          `Error on coalecing plugin state, on field: ${field}`
          + `. Affected plugins: ${plugins}`
          + `. State: ${state}`, ys
        );
        return [];
      })
    ),
    A.reduce([], ModelOverwrite)
  ),
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
