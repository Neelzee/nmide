import "@nmide/js-utils";
import { pipe } from "fp-ts/lib/function";
import { reduce } from "fp-ts/ReadonlyArray";
import { make } from "fp-ts/Const";
import * as A from "fp-ts/Array";
import * as O from "fp-ts/Option";
import * as E from "fp-ts/Either";
import * as TE from "fp-ts/TaskEither";
import * as T from "fp-ts/Task";
import { Core, CoreModification, EventHandler } from "@nmide/js-utils/lib/Core";
import { DCoreModification, DValueArr } from "@nmide/js-utils/lib/Decoder";
import { Errors } from "io-ts";
import { coreEvaluation } from "./App";

// TODO: This is complicated, should be rewritten
// Split different tasks into different functions.
// This is complex! And I am not sure it works!
// This needs to be changed, everytime an event is added or removed, that
// specific Event needs to be properly cleaned up. This depends on the
// backend implementation. For example, with Tauri, we are using the
// builtin Event-System, inwhich all `listen`-functions, return a cleanup
// function, for _unlistening_ to the specified event. This could be a
// criteria we extend to all backends.
export const runtime = async (task: T.Task<Core>) => pipe(
  task,
  T.map(core => {
    window.core = core;
    return window.core;
  }),
  T.map(({ events, eventHandlers }) => pipe(
    events.kids,
    A.map(({ event, module }) => pipe(
      eventHandlers.get(event),
      O.fromNullable,
      O.match<EventHandler[], EventHandler[]>(
        () => {
          window.log.info(
            `No EventHandlers for Event: ${event}, from module: ${module}`
          );
          return [];
        },
        make,
      ),
      xs => window.listen(event, ({ payload }) => pipe(
        payload === undefined
          ? E.right([])
          : DValueArr.decode(payload),
        E.match(
          err => {
            window.log.error(
              `Error when decoding event arguments from Event: ${event}`
              + `, module: ${module}`
              + `, error:`, err
            );
            return [];
          },
          args => pipe(
            xs,
            A.map(({ handler, module }) => pipe(
              TE.tryCatch(
                () => handler(window.core, ...args),
                err => new Error(
                  `Exception thrown from module: ${module}`
                  + `, error: ${JSON.stringify(err)}`
                ),
              ),
              TE.match(
                err => {
                  window.log.error(err);
                  return {
                    uiModifications: [],
                    stateModifications: [],
                    eventModifications: [],
                    newEventHandlers: [],
                  };
                },
                u => pipe(
                  u,
                  DCoreModification.decode,
                  E.match<Errors, CoreModification, CoreModification>(
                    errs => {
                      window.log.error(
                        new Error(
                          `Error when decoding core modifications`
                          + `, from module: ${module}`
                        ),
                        errs
                      );
                      return {
                        uiModifications: [],
                        stateModifications: [],
                        eventModifications: [],
                        newEventHandlers: [],
                      };
                    },
                    make,
                  ),
                ),
              ),
            )),
          ),
        ),
        T.sequenceArray,
        T.map(reduce(window.core, coreEvaluation)),
        T.map(core => { window.core = core; }),
        task => task(),
      )),
    )),
  )),
);
