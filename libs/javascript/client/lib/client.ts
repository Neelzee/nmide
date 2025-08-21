/**
 * @package client
 *
 * Client for communicating with the backend, with runtime typechecking
 * and TypeScript type information included.
 *
 * @author Nils Michael <nilsien2001@gmail.com>
 */

import {
  invoke,
  type InvokeArgs,
  type InvokeOptions
} from "@tauri-apps/api/core";
import { type CoreModification, type Event } from "@nmide/js-utils";
import * as E from "fp-ts/Either";
import * as t from "io-ts";
import { pipe } from "fp-ts/lib/function";
import { DState, DHtml, DCoreModification } from "@nmide/js-decoder-lib";
import { formatValidationErrors } from "io-ts-reporters";

/**
 * Type family that maps arguments to the corresponding expected argument.
 */
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

/**
 * Mapping between the expected type from an `invoke`-ation, and the _cmd_.
 *
 * @see [Tauri Invoke](https://tauri.app/develop/calling-rust/)
 */
export const ClientDecodedType = {
  init: t.null,
  handler: t.null,
  state: DState,
  ui: DHtml,
  modification: DCoreModification,
}

/**
 * Map that transforms the validation errors to list of readable errors
 * (strings).
 */
export const ClientDecoder = {
  init: (e: unknown) => E.mapLeft(formatValidationErrors)(t.null.decode(e)),
  handler: (e: unknown) => E.mapLeft(formatValidationErrors)(t.null.decode(e)),
  state: (e: unknown) => E.mapLeft(formatValidationErrors)(DState.decode(e)),
  ui: (e: unknown) => E.mapLeft(formatValidationErrors)(DHtml.decode(e)),
  modification: (e: unknown) =>
    E.mapLeft(formatValidationErrors)(DCoreModification.decode(e)),
}

/**
 * Type family where the invoke `cmd` is the key, allowing for a typesafe
 * client.
 */
type ClientDecodedType<K extends keyof ClientArgs & keyof typeof ClientDecoder>
  = t.TypeOf<(typeof ClientDecodedType)[K]>;

/**
 * Client for communicating with the backend.
 * 
 * @param cmd Name of the Rust function being invoked
 * @param args Semi-optional argument, is optional to aoiv having to pass
 * `undefined` to functions without arguments, but if args is not defined for an
 * endpoint that requires an argument, it will fail
 * @param options Invoke options
 *
 *
 * @description In the following diagram, we can see how the communication
 * between the frontend and backend are.
 *
 *    ┌──────────┐
 *    │ Frontend │◄┐
 *  ┌─└──────────┘ │
 *  │              │
 *  │ ┌─────────┐  │
 *  └►│ Backend │──┘
 *    └─────────┘
 *
 * Its a simple request-response, the arguments being passed, JSON data. But,
 * the way the system is set up, makes the backend the arbiter of what is right.
 * This is due to Rust typesystem. But there might be breaking changes between
 * the backend and frontend types, which, using the standard `invoke` method
 * provided by Tauri, will result in an _simple_ error message about how the
 * arguments are not correct. Using this client, however, allows us to do
 * additional decoding using the `io-ts` library, allowing us to do runtime
 * type validation. We have also added typing information to the client, meaning
 * when we invoke the client with the arguments `client("init")`, TypeScript
 * knows that this function returns the type `Promise<E.Either<Error, void>>`,
 * and requires no arguments. (Or just undefined). This runtime typechecking is
 * useful when the API is unstable, as it is under development, but could still
 * be useful, if we choose to allow modules use this client aswell.
 *
 * @see [Tauri Invoke](https://tauri.app/develop/calling-rust/)
 * @see [io-ts](https://github.com/gcanti/io-ts/blob/master/index.md)
 */
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
