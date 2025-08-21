import { emptyCm, type Core, type Module, type ModuleUnknown } from "@nmide/js-utils";
import { pipe } from "fp-ts/lib/function";
import * as E from "fp-ts/Either";
import * as TE from "fp-ts/TaskEither";
import * as T from "fp-ts/Task";
import { DCoreModification } from "@nmide/js-decoder-lib";
import { formatValidationErrors } from "io-ts-reporters";
import { task } from "fp-ts";

const moduleWrapper = (m: ModuleUnknown): Module => {
  return {
    ...m,
    init: (core: Core) => pipe(
      TE.tryCatch(
        () => m.init(core),
        err => {
          window.__nmideConfig__
            .log
            .error(`Error on Module.init from module: ${m.name}, error: ${JSON.stringify(err)}`);
          return;
        },
      ),
      TE.map(DCoreModification.decode),
      TE.getOrElse(() => task.of(E.right(emptyCm()))),
      TE.mapLeft(formatValidationErrors),
      TE.getOrElse(errs => {
        window.__nmideConfig__
          .log
          .error(`Error when parsing result from Module.init from module: ${m.name}, error: ${JSON.stringify(errs)}`);
        return T.of(emptyCm());
      }),
      task => task(),
    ),
    handler: (event: Event, core: Core) => pipe(
      TE.tryCatch(
        () => m.handler(event, core),
        err => {
          window.__nmideConfig__
            .log
            .error(`Error on Module.handler from module: ${m.name}, error: ${JSON.stringify(err)}`);
          return;
        },
      ),
      TE.map(DCoreModification.decode),
      TE.getOrElse(() => task.of(E.right(emptyCm()))),
      TE.mapLeft(formatValidationErrors),
      TE.getOrElse(errs => {
        window.__nmideConfig__
          .log
          .error(`Error on decoding result from Module.handler from module: ${m.name}, error: ${JSON.stringify(errs)}`);
        return T.of(emptyCm());
      }),
      task => task(),
    )
  }
};

export default moduleWrapper;