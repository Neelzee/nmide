import * as A from "fp-ts/Array";
import { pipe } from "fp-ts/lib/function";
import { Box, Ins, isModIns, ModIns, RemIns } from "./instruction";

// TODO: Add docs
export const evalTree = <T extends object, G extends keyof T = keyof T>(b: Box<T>, ins: Ins<T, G>[]) => pipe(
  ins,
  A.reduce(b, (accBox, ins) => isModIns<T, G>(ins)
    ? modifyInstruction<T, G>(accBox, ins)
    : removeInstruction<T>(accBox, { f: ins.f })
  )
);

const removeInstruction = <T extends object>(
  { box, getKids, addKids }: Box<T>,
  { f }: RemIns<T>
): Box<T> => pipe(
  { box, getKids, addKids },
  getKids,
  A.filter(({ box: b }) => !f(b)),
  A.map(k => removeInstruction(k, { f })),
  xs => addKids({ box, getKids, addKids }, ...xs),
);

const modifyInstruction = <
  T extends object,
  G extends keyof T = keyof T,
>(
  { box, getKids, addKids }: Box<T>,
  { f, field, g }: ModIns<T, G>
): Box<T> => f(box)
    ? pipe(
      box,
      box => {
        box[field] = g(box[field]);
        return { box, getKids, addKids };
      },
    )
    : pipe(
      getKids({ box, getKids, addKids }),
      A.map(k => modifyInstruction(k, { f, field, g })),
      kids => addKids({ box, addKids, getKids }, ...kids),
    );
