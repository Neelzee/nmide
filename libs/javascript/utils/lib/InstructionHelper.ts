/// TODO: Add docs

/// TODO: Add docs
import { attrsNanCheck } from "./AttrUtils";
import { Html } from "./Html";
import { addAttr, addKid, applyById, getElementById, hasId, htmlNanCheck, kids, remAttr, replaceKids, setText } from "./HtmlUtils";
import type { Instruction } from "./Instruction";
import type { Attr } from "./Attr.ts";
import { Value } from "./Value";
import { valueNanCheck } from "./ValueUtils";
import { State } from "./State.ts";
import { pipe } from "fp-ts/lib/function";

export const combine = <T>(a: Instruction<T>, b: Instruction<T>): Instruction<T> => {
  if (a === "noOp") return b;
  if (b === "noOp") return a;
  return { then: [a, b] }
}

export const applyUiInstr = ([n, s, a]: [Instruction<Html>, Instruction<string>, Instruction<Attr>]) => (ui: Html): Html => {
  if (isAdd(n)) {
    const [f, v] = n.add;
    if (f === "") {
      ui = addKid(ui, v);
    } else {
      ui = applyById(f)(p => addKid(p, v))(ui);
    }
  } else if (isRem(n)) {
    const [f, _] = n.rem;
      ui = applyById(f)(p => replaceKids(p, kids(p).filter(k => !hasId(f)(k))))(ui);
  } else if (!isNoOp(n)) {
    const [fst, snd] = n.then;
    ui = pipe(
      ui,
      applyUiInstr([fst, s, a]),
      applyUiInstr([snd, s, a]),
    )
  }

  if (isAdd(s)) {
    const [f, v] = s.add;
    if (f === "") {
      ui = setText(ui, v);
    } else {
      ui = applyById(f)(p => setText(p, v))(ui);
    }
  } else if (isRem(s)) {
    const [f, _] = s.rem;
      ui = applyById(f)(p => setText(p, ""))(ui);
  } else if (!isNoOp(s)) {
    const [fst, snd] = s.then;
    ui = pipe(
      ui,
      applyUiInstr([n, fst, a]),
      applyUiInstr([n, snd, a]),
    )
  }

  if (isAdd(a)) {
    const [f, v] = a.add;
    if (f === "") {
      ui = addAttr(ui, v);
    } else {
      ui = applyById(f)(p => addAttr(p, v))(ui);
    }
  } else if (isRem(a)) {
    const [f, v] = a.rem;
      ui = applyById(f)(p => remAttr(p, v))(ui);
  } else if (!isNoOp(a)) {
    const [fst, snd] = a.then;
    ui = pipe(
      ui,
      applyUiInstr([n, s, fst]),
      applyUiInstr([n, s, snd]),
    )
  }

  return ui;
}

export const applyStateInstr = (i: Instruction<Value>) => (state: State): State => {
  if (isNoOp(i)) return state;
  if (isAdd(i)) {
    const [f, v] = i.add;
    state[f] = v;
    return state;
  }
  if (isRem(i)) {
    const [f, _] = i.rem;
    state[f] = undefined;
    return state;
  }
  const [fst, snd] = i.then;
  return pipe(
    state,
    applyStateInstr(fst),
    applyStateInstr(snd),
  )
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
