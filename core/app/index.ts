import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getHtml, renderHtml, THtml } from "./lib/renderHtml";
import { Html, TUIInstruction } from "@nmide/js-utils/lib/Html";
import { TAttr } from "@nmide/js-utils";
import { pipe } from "fp-ts/lib/function";
import * as A from "fp-ts/Array";

document.addEventListener("DOMContentLoaded", () => {
  window.root = document.body;
  listen("counter", event => {
    console.log(event.payload);
  });
  listen<{ inst: TUIInstruction["op"], ui: Html }>("nmide://render", event => {
    const { inst, ui } = event.payload;
    console.log(event);
    // TODO: Do actual VDOM stuff
    document.body.textContent = "";
    const html = parseInst({ op: inst }, getHtml(ui));
    renderHtml(html);
  });
  invoke<{ inst: TUIInstruction["op"], ui: Html }>("init").then(({ inst, ui }) => {
    console.log({ inst, ui });
    const html = parseInst({ op: inst }, { kind: "main", kids: [], attrs: [] });
    renderHtml(html);
  });
});

const parseInst = ({ op }: TUIInstruction, root: THtml): THtml => {
  if (op === "noOp") {
    return root;
  }
  if ("addAttr" in op) {
    return root;
  }
  if ("add" in op) {
    const { ui, id, class: cls } = op.add;
    const f = (parent: THtml) => { return { ...parent, kids: [...parent.kids, getHtml(ui)] }; };
    if (id !== null) {
      return modifyTHtml(root, f, (p: THtml) => hasId(p.attrs, id));
    }
    if (cls !== null) {
      return modifyTHtml(root, f, (p: THtml) => hasClass(p.attrs, cls));
    }
    return { ...root, kids: [...root.kids, getHtml(ui)] };
  }
  if ("textAttrPred" in op) {
    const { id, class: cls, text } = op.textAttrPred;
    const f = (n: THtml) => { return { ...n, text: text }; };
    if (id !== null) {
      return modifyTHtml(root, f, (p: THtml) => hasId(p.attrs, id));
    }
    if (cls !== null) {
      return modifyTHtml(root, f, (p: THtml) => hasClass(p.attrs, cls));
    }
  }
  if ("then" in op) {
    const { fst, snd } = op.then;
    const new_root = parseInst({ op: fst }, root);
    return parseInst({ op: snd }, new_root);
  }
  console.debug("No parse for instruction: ", op);
  return root;
};

const hasClass = (xs: TAttr[], cls: string): boolean => pipe(
  xs,
  A.filter(el => "class" in el),
  A.reduce(false, (acc, e) => acc || e.class === cls)
)

const hasId = (xs: TAttr[], id: string): boolean => pipe(
  xs,
  A.filter(el => "id" in el),
  A.reduce(false, (acc, e) => acc || e.id === id)
);

const modifyTHtml = (ui: THtml, f: ((h: THtml) => THtml), p: ((h: THtml) => boolean)): THtml => {
  if (p(ui)) {
    return f(ui);
  }
  return { ...ui, kids: ui.kids.map(k => modifyTHtml(k, f, p)) };
};
