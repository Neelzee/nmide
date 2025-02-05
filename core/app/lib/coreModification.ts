import "@nmide/js-utils";
import {
  emptyHtml,
  THtml,
  THtmlEq,
} from "@nmide/js-utils";
import { PathReporter } from "io-ts/PathReporter";
import { pipe } from "fp-ts/lib/function";
import * as A from "fp-ts/Array";
import * as M from "fp-ts/Map";
import * as E from "fp-ts/Either";
import * as O from "fp-ts/Option";
import * as TE from "fp-ts/TaskEither";
import * as T from "fp-ts/Task";
import * as RA from "fp-ts/ReadonlyArray";
import { ModuleUnknown as Module } from "@nmide/js-utils/lib/Module";
import { Core, CoreModification, CoreModificationUnknown, EventHandler, } from "@nmide/js-utils/lib/Core";
import { DCoreModification } from "@nmide/js-utils/lib/Decoder";
import { Box, isModIns } from "@nmide/js-utils/lib/instruction";
import { evalTree } from "@nmide/js-utils/lib/evalTree";
import { parseHtml } from "./renderHtml";

export const coreModifications = (modules: [string, Module][]) => pipe(
  modules,
  A.map(safeWrapModule),
  A.map(decodeTask),
  A.map(TE.getOrElse(err => {
    window.log.error(err);
    return T.of(emptyCoreModification());
  })),
  T.sequenceArray,
  T.map(RA.map(render(window.core))),
  T.map(RA.reduce(window.core, coreEvaluation)),
);


const traverse = (x: THtml, f: (x: THtml) => boolean): O.Option<THtml> => f(x)
  ? O.some(x)
  : pipe(
    x.kids,
    A.findFirst(y => O.isSome(traverse(y, f))),
  );

// NOTE: Effectful function
export const render = (core: Core) => (cm: CoreModification): CoreModification => pipe(
  cm.uiModifications,
  A.map(ins => isModIns(ins)
    ? pipe(
      ins,
      ({ f, field, g }) => pipe(
        field,
        fld => {
          const [element, thElement] = pipe(
            traverse(core.ui, f),
            O.getOrElse(() => {
              window.log.error("Could not find thtml");
              return emptyHtml();
            }),
            k => pipe(
              window.uiMap,
              M.lookup(THtmlEq)(k),
              O.match<HTMLElement, [HTMLElement, THtml]>(
                () => {
                  window.log.error("Could not find any matching HTML from: ", k);
                  return [window.document.createElement("div"), k];
                },
                element => [element, k],
              ),
            ),
            ([a, b]) => pipe(
              b,
              x => {
                return { ...x, field: g(x[field]) };
              },
              x => [a, x]
            ),
          );
          switch (fld) {
            case ("kind"): {
              const newElement = window.document.createElement(thElement.kind);
              // NOTE: What is this?
              const props = Array.prototype.slice.call(element.attributes);
              pipe(
                props,
                A.map(attr => newElement.setAttribute(attr.nodeName, attr.nodeValue))
              );
              const parent = element.parentNode;
              if (parent === null) {
                window.log.error("Parent of element is null", thElement);
              }
              parent?.appendChild(newElement);
              parent?.removeChild(element);
            }
              break;
            case ("kids"): {
              const kids = pipe(
                thElement.kids,
                A.map(parseHtml),
              );
              element.childNodes.forEach(v => element.removeChild(v));
              kids.forEach(k => element.appendChild(k));
            }
              break;
            case ("text"):
              element.textContent = thElement.text;
              break;
            case ("attrs"):
              // TODO: Implement
              window.log.error("Not Implemented");
              break;
            default:
              window.log.error("Unknown field on thtml: ", field);
              break;
          }
        }
      ),
    )
    : pipe(
      traverse(core.ui, ins.f),
      O.getOrElse(() => {
        window.log.error("Could not find thtml");
        return emptyHtml();
      }),
      k => pipe(
        window.uiMap,
        M.lookup(THtmlEq)(k),
        O.match(
          () => {
            window.log.error("Could not find any matching HTML from: ", k);
          },
          element => {
            if (window.root.removeChild(element) === undefined) {
              window.log.error(
                "Found a match, but could not remove element",
                element
              );
            }
          },
        ),
      ),
    )
  ),
  () => cm,
);

const getKids = ({ box: { kids }, ...rem }: Box<THtml>): Box<THtml>[] => pipe(
  kids,
  A.map(box => {
    return { box, ...rem };
  }),
);

const addKids = (
  { box: { kids: _, ...xs }, ...ys }: Box<THtml>,
  ...boxes: Box<THtml>[]
): Box<THtml> => {
  return {
    ...ys,
    box: {
      kids: pipe(
        boxes,
        A.map(({ box }) => box),
      ), ...xs
    },
  }
}

// TODO: Add state modification
// TODO: Add event modification
export const coreEvaluation = (
  core: Core,
  coreModifications: CoreModification
): Core => {
  console.log("bar:", core.ui);
  const foo = evalTree({ box: core.ui, getKids, addKids }, coreModifications.uiModifications);
  console.log("foo:", foo.box);
  return {
    ...core,
    ui: pipe(
      foo,
      ({ box }) => box
    ),
    eventHandlers: ((): Map<string, EventHandler[]> => {
      const map = core.eventHandlers;
      return pipe(
        coreModifications.newEventHandlers,
        // NOTE: Closest I know how to verify that the given handler is valid.
        // Atleast during runtime. Checking that the function accept atleast one
        // argument.
        A.filter(([_, { handler }]) => handler.length >= 1),
        A.map(([e, h]) => {
          let handlers = map.get(e);
          if (handlers === undefined) {
            handlers = [];
          }
          // HACK: Is this true? Don't know, but it needs to be true to fit
          // with the typechecking
          handlers.push(h as EventHandler)
          map.set(e, handlers);
        }),
        _ => map
      );
    })(),
  };
}


// TODO: Add verification
// TODO: Encode unverified eventhandler into the CoreModicitation type
/**
 * Verifies that the given modification is valid.
 * io-ts cannot properly validate the output from a Module, because I cannot
 * type a function more accurately than, it is a function. But using things like
 * `.length`, I can count the arguments, so, still after the _verification_ it
 * still might be invalid. But this only pertains to the `EventHandler`s.
 */
export const verify = (
  cm: CoreModificationUnknown
): cm is CoreModification => true;

/**
 * Turns a module initialization from a promise that _might_ be valid, to one
 * that _is_ valid, i.e. calling it will not throw an exception.
 */
export const safeWrapModule = (
  [module, { init }]: [string, Module]
): [string, TE.TaskEither<Error, unknown>] => pipe(
  TE.tryCatch(
    () => init(window.core),
    reason => new Error(
      `Exception on init from module: ${module}`
      + `, errors: ${JSON.stringify(reason)}`
    ),
  ),
  task => [module, task]
);

/**
 * Decodes the unknown value from a module, resulting in Either a verified
 * CoreModification, or an Error
 */
export const decodeTask = (
  [module, task]: [string, TE.TaskEither<Error, unknown>]
): TE.TaskEither<Error, CoreModification> => pipe(
  task,
  TE.match(
    E.left,
    u => pipe(
      u,
      DCoreModification.decode,
      decoded => E.isLeft(decoded)
        ? E.left(
          new Error(
            `Error on decoding core modifications from module: ${module}`
            + `, supplied object: ${JSON.stringify(u)}`
            + `, errors:  ${JSON.stringify(PathReporter.report(decoded))}`
          )
        )
        : decoded,
      E.map(cm => verify(cm)
        ? cm
        : emptyCoreModification())
    )
  ),
);

export const emptyCoreModification = (): CoreModification => {
  return {
    uiModifications: [],
    stateModifications: [],
    eventModifications: [],
    newEventHandlers: []
  }
};
