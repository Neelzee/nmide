/// TODO: Add docs

import { Instruction } from "./Instruction";

export const combine = <T>(a: Instruction<T>, b: Instruction<T>): Instruction<T> => {
  if (a === "noOp") return b;
  if (b === "noOp") return a;
  return { then: [a, b] }
}