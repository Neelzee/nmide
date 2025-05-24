/// TODO: Add docs

/// TODO: Add docs
import type { Instruction } from "./Instruction";

export const combine = <T>(a: Instruction<T>, b: Instruction<T>): Instruction<T> => {
  if (a === "noOp") return b;
  if (b === "noOp") return a;
  return { then: [a, b] }
}

export const isNoOp = <T>(instr: Instruction<T>): instr is "noOp" =>
  typeof instr === "string" && instr === "noOp";


export const isAdd = <T>(instr: Instruction<T>): instr is Extract<Instruction<T>, { add: [string, T] }> =>
  typeof instr === "object" && instr !== null && "add" in instr;

export const isRem = <T>(instr: Instruction<T>): instr is Extract<Instruction<T>, { rem: [string, T] }> =>
  typeof instr === "object" && instr !== null && "rem" in instr;

export const flatten = <T>(instr: Instruction<T>): Exclude<Exclude<Instruction<T>, "noOp">, { then: [Instruction<T>, Instruction<T>] }>[] => {
  if (isNoOp(instr)) return [];
  if (isAdd(instr) || isRem(instr)) return [instr];
  const [fst, snd] = instr.then;
  return [...flatten(fst), ...flatten(snd)];
}
