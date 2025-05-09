import { listen, type EventCallback, type UnlistenFn } from "@tauri-apps/api/event";
import { type Event, type Html, type Instruction, type Attr } from "@nmide/js-utils";
import * as E from "fp-ts/Either";
import * as t from "io-ts";
import { DEvent, DRenderEvent } from "./decoder";
import { pipe } from "fp-ts/lib/function";

export type ListenArgs = {
  "nmide://render": [Instruction<Html>, Instruction<string>, Instruction<Attr>],
  "nmide://event": Event,
}

export const ListenDecoder = {
  "nmide://render": DRenderEvent,
  "nmide://event": DEvent
}

type ListenDecodedType<K extends keyof ListenArgs & keyof typeof ListenDecoder>
  = t.TypeOf<(typeof ListenDecoder)[K]>;

export const Listen = <
  K extends keyof ListenArgs & keyof typeof ListenDecoder,
  A extends ListenArgs[K]
>(
  cmd: K,
  handler: EventCallback<A>,
): Promise<E.Either<Error, UnlistenFn>> => 
  listen<A>(cmd, event => pipe(
    event.payload,
    ListenDecoder[cmd].decode,
    E.match<t.Errors, ListenDecodedType<K>, E.Either<Error, void>>(
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
  