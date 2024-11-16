import { invoke, InvokeArgs, InvokeOptions } from "@tauri-apps/api/core"
import * as t from "io-ts";
import * as E from "fp-ts/Either";
import { pipe } from "fp-ts/lib/function";
import { PathReporter } from "io-ts/PathReporter";
import { TMap, Decoder, TMsg } from "@nmide/js-utils";

const { DHtmlArr, DInitDecoder, DUpdateDecoder } = Decoder;

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
    args: { plugin_name: string },
  },
  "plugin_view": {
    args: { plugin_name: string, tmodel: TMap },
  },
  "plugin_update": {
    args: { plugin_name: string, tmsg: TMsg, tmodel: TMap, },
  },
  "get_plugins": {
    args: undefined,
  }
}

export const NmideDecoder = {
  "init": DInitDecoder,
  "view": DHtmlArr,
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

export const NmideInvoker = <
  K extends keyof NmideArgs & keyof typeof NmideDecoder,
  A extends InvokeArgs & NmideArgs[K]["args"]
>(
  cmd: K,
  args?: A,
  options?: InvokeOptions,
): Promise<E.Either<Error, unknown>> =>
  invoke(cmd, args, options)
    .then(e => {
      console.debug("Value: ", e);
      return E.right(e);
    })
    .catch(err => E.left(new Error(err)));


const NmideClient = <
  K extends keyof NmideArgs & keyof typeof NmideDecoder,
  A extends InvokeArgs & NmideArgs[K]["args"]
>(
  cmd: K,
  args?: A,
  options?: InvokeOptions,
): Promise<E.Either<Error, NmideDecodedType<K>>> =>
  NmideInvoker(cmd, args, options)
    .then(unknown_data => pipe(
      unknown_data,
      NmideDecoder[cmd].decode,
      E.match<t.Errors, NmideDecodedType<K>, E.Either<Error, NmideDecodedType<K>>>(
        errs => E.left(
          new Error(
            `Got errors validating results from backend on cmd: ${cmd}, Validation Errors: ${errs.map(e => JSON.stringify(e)).join("\n")}`
          )
        ),
        data => E.right(data)
      ),
    ));

export default NmideClient;
