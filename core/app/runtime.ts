import "@nmide/js-utils";
import { pipe } from "fp-ts/lib/function";
import * as A from "fp-ts/Array";
import * as E from "fp-ts/Either";
import { EventHandler, CoreModification } from "@nmide/js-utils/lib/Core";
import { coreEvaluation, emptyCoreModification, verify } from "./lib/coreModification";
import { DCoreModification } from "@nmide/js-utils/lib/Decoder";

// NOTE: Side-effects
export const eventHandler = () => window.listen("event", ({ payload }) => {
  if (payload !== null && typeof payload === "object" && "event" in payload) {
    const { event, ...args } = payload;
    if (event !== null && typeof event === "string") {
      const handlers = window.eventHandlers.get(event);
      if (handlers !== undefined) {
        window.core = pipe(
          handlers,
          A.map<EventHandler, unknown>(h => {
            try {
              // HACK
              // @ts-expect-error I dont care about validation
              return h.handler(window.core, args);
            } catch (err) {
              window.log.error(`Error on EventHandler, from module ${h.module}`, err);
              return {};
            }
          }),
          A.map<unknown, E.Either<Error, CoreModification>>(u => pipe(
            u,
            DCoreModification.decode,
            E.mapLeft(
              errs => new Error(
                `Error on decoding core modifications`
                + `, errors:  ${JSON.stringify(errs)}`
              )
            ),
            E.map(cm => verify(cm)
              ? cm
              : emptyCoreModification())
          )),
          A.map<E.Either<Error, CoreModification>, CoreModification>(E.getOrElse(err => {
            window.log.error(err);
            return emptyCoreModification();
          })),
          A.reduce(window.core, coreEvaluation),
          c => window.core = c,
        );
      }
    }
  }
});
