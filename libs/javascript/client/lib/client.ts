import { invoke, type InvokeArgs, type InvokeOptions } from "@tauri-apps/api/core";
import { type CoreModification, type Event } from "@nmide/js-utils";
import * as E from "fp-ts/Either";
import * as t from "io-ts";
import { pipe } from "fp-ts/lib/function";
import { DState, DHtml, DCoreModification } from "@nmide/js-decoder-lib";
import { formatValidationErrors } from "io-ts-reporters";

export type ClientArgs = {
  init: {
    args: undefined,
  },
  handler: {
    args: {
      event: Event,
    },
  },
  state: {
    args: undefined,
  },
  ui: {
    args: undefined,
  },
  modification: {
    args: {
      modification: CoreModification
    }
  }
}

export const ClientDecodedType = {
  init: t.void,
  handler: t.void,
  state: DState,
  ui: DHtml,
  modification: DCoreModification,
}

export const ClientDecoder = {
  init: (e: unknown) => E.mapLeft(formatValidationErrors)(t.void.decode(e)),
  handler: (e: unknown) => E.mapLeft(formatValidationErrors)(t.void.decode(e)),
  state: (e: unknown) => E.mapLeft(formatValidationErrors)(DState.decode(e)),
  ui: (e: unknown) => E.mapLeft(formatValidationErrors)(DHtml.decode(e)),
  modification: (e: unknown) => E.mapLeft(formatValidationErrors)(DCoreModification.decode(e)),
}

type ClientDecodedType<K extends keyof ClientArgs & keyof typeof ClientDecoder>
  = t.TypeOf<(typeof ClientDecodedType)[K]>;

export const Client = <
  K extends keyof ClientArgs & keyof typeof ClientDecoder,
  A extends InvokeArgs & ClientArgs[K]["args"]
>(
  cmd: K,
  args?: A,
  options?: InvokeOptions,
): Promise<E.Either<Error, ClientDecodedType<K>>> =>
  invoke(cmd, args, options)
    .then(E.right)
    .catch(err => E.left(new Error(err)))
    .then(
      E.match<Error, unknown, E.Either<Error, ClientDecodedType<K>>>(
        E.left,
        unknown_data => pipe(
          unknown_data,
          ClientDecoder[cmd],
          E.match<string[], ClientDecodedType<K>, E.Either<Error, ClientDecodedType<K>>>(
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
