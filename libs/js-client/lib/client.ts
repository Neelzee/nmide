import { invoke, type InvokeArgs, type InvokeOptions } from "@tauri-apps/api/core";
import { type CoreModification, type Event } from "@nmide/js-utils";
import * as E from "fp-ts/Either";
import * as t from "io-ts";
import { DHtml, DState } from "./decoder";
import { pipe } from "fp-ts/lib/function";

export type ClientArgs = {
  init: {
    args: {
      mods: CoreModification[],
    },
  },
  handler: {
    args: {
      event: Event,
      mods: CoreModification[],
    },
  },
  state: {
    args: undefined,
  },
  ui: {
    args: undefined,
  }
}

export const ClientDecoder = {
  init: t.void,
  handler: t.void,
  state: DState,
  ui: DHtml,
}

type ClientDecodedType<K extends keyof ClientArgs & keyof typeof ClientDecoder>
  = t.TypeOf<(typeof ClientDecoder)[K]>;

const InvokerWrapper = <K extends keyof ClientArgs & keyof typeof ClientDecoder, A extends InvokeArgs & ClientArgs[K]["args"]>(
  cmd: K,
  args?: A,
  options?: InvokeOptions,
): Promise<E.Either<Error, unknown>> => invoke(cmd, args, options).then(E.right).catch(err => E.left(new Error(err)));

export const Client = <
  K extends keyof ClientArgs & keyof typeof ClientDecoder,
  A extends InvokeArgs & ClientArgs[K]["args"]
>(
  cmd: K,
  args?: A,
  options?: InvokeOptions,
): Promise<E.Either<Error, ClientDecodedType<K>>> => InvokerWrapper(cmd, args, options).then(
  E.match<Error, unknown, E.Either<Error, ClientDecodedType<K>>>(
    E.left,
    unknown_data => pipe(
      unknown_data,
      ClientDecoder[cmd].decode,
      E.match<t.Errors, ClientDecodedType<K>, E.Either<Error, ClientDecodedType<K>>>(
        errs => E.left(
          new Error(
            `Error from validating backend: ${JSON.stringify(errs)}`
            + `, supplied data: ${JSON.stringify(unknown_data)}`
          )
        ),
        data => E.right(data),
      ),
    )
  )
);
