import { invoke, InvokeArgs, InvokeOptions } from "@tauri-apps/api/core"
import * as t from "io-ts";
import * as E from "fp-ts/Either";
import { pipe } from "fp-ts/lib/function";
import { TMap, Decoder, TMsg } from "@nmide/js-utils";
import { NmDebugLogMsg } from "@nmide/js-utils/lib/Debug";

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
    .then(E.right)
    .catch(err => E.left(new Error(err)));


const NmideClient = <
  K extends keyof NmideArgs & keyof typeof NmideDecoder,
  A extends InvokeArgs & NmideArgs[K]["args"]
>(
  cmd: K,
  args?: A,
  options?: InvokeOptions,
) => NmideInvoker(
  cmd,
  args,
  options,
).then(
  E.match<Error, unknown, E.Either<Error, NmideDecodedType<K>>>(
    E.left,
    unknown_data => pipe(
      unknown_data,
      NmideDecoder[cmd].decode,
      E.match<t.Errors, NmideDecodedType<K>, E.Either<Error, NmideDecodedType<K>>>(
        errs => E.left(
          new Error(`Error from validating backend: ${JSON.stringify(errs)}`)
        ),
        data => E.right(data),
      ),
    )
  )
);

export default NmideClient;
