/// TODO: Add docs

/// TODO: Add docs
import { attrsNanCheck } from "./AttrUtils";
import { Html } from "./Html";
import { htmlNanCheck } from "./HtmlUtils";
import type { Instruction } from "./Instruction";
import type { Attr } from "./Attr.ts";
import { Value } from "./Value";
import { valueNanCheck } from "./ValueUtils";

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

export const nanCheck = (i: Instruction<Value>): Instruction<Value> => {
  if (isNoOp(i)) return i;
  if (isRem(i)) return i;
  if (isAdd(i)) {
    const [k, v] = i.add;
    return { add: [k, valueNanCheck(v)] };
  }
  const [fst, snd] = i.then;
  return { then: [nanCheck(fst), nanCheck(snd)] };
};


export const uiNanCheck = (i: Instruction<Html>): Instruction<Html> => {
  if (isNoOp(i)) return i;
  if (isRem(i)) return i;
  if (isAdd(i)) {
    const [k, v] = i.add;
    return { add: [k, htmlNanCheck(v)] };
  }
  const [fst, snd] = i.then;
  return { then: [uiNanCheck(fst), uiNanCheck(snd)] };
}

export const attsInstrNanCheck = (i: Instruction<Attr>): Instruction<Attr> => {
  if (isNoOp(i)) return i;
  if (isRem(i)) return i;
  if (isAdd(i)) {
    const [k, v] = i.add;
    return { add: [k, attrsNanCheck(v)] };
  }
  const [fst, snd] = i.then;
  return { then: [attsInstrNanCheck(fst), attsInstrNanCheck(snd)] };
}
