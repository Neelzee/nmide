/**
 * @package listener
 *
 * Wrapper around Tauris listen method. Includes runtime typechecking and
 * TypeScript type information mapped on the given eventName. See the
 * `client.ts` file for more information, as the listener is implemented in a
 * similar manner.
 *
 */

import { listen, type EventCallback, type UnlistenFn } from "@tauri-apps/api/event";
import { type Event, type Html, type Instruction, type Attr } from "@nmide/js-utils";
import * as E from "fp-ts/Either";
import * as t from "io-ts";
import { pipe } from "fp-ts/lib/function";
import { formatValidationErrors } from "io-ts-reporters";
import { DEvent, DRenderEvent } from "@nmide/js-decoder-lib";

export type ListenArgs = {
  "nmide://render": [Instruction<Html>, Instruction<string>, Instruction<Attr>],
  "nmide://event": Event,
}

export const ListenDecoder = {
  "nmide://render": (e: unknown) => E.mapLeft(formatValidationErrors)(DRenderEvent.decode(e)),
  "nmide://event": (e: unknown) => E.mapLeft(formatValidationErrors)(DEvent.decode(e))
}

export const ListenDecoderType = {
  "nmide://render": DRenderEvent,
  "nmide://event": DEvent
}

type ListenDecodedType<K extends keyof ListenArgs & keyof typeof ListenDecoder>
  = t.TypeOf<(typeof ListenDecoderType)[K]>;

export const Listen = <
  K extends keyof ListenArgs & keyof typeof ListenDecoder,
  A extends ListenArgs[K]
>(
  cmd: K,
  handler: EventCallback<A>,
): Promise<E.Either<Error, UnlistenFn>> =>
  listen<A>(cmd, event => pipe(
    event.payload,
    ListenDecoder[cmd],
    E.match<string[], ListenDecodedType<K>, E.Either<Error, void>>(
      errs => E.left(
        new Error(
          `Error from validating event: ${JSON.stringify(errs)}`
          + `, supplied event: ${JSON.stringify(event)}`
        )
      ),
      _ => E.right(handler(event)),
    ),
  ),
  ).then(E.right)
    .catch(err => E.left(new Error(err)));
