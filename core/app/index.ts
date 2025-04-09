import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getHtml, parseHtml, THtml } from "./lib/renderHtml";
import { TUIInstruction } from "@nmide/js-utils/lib/Html";
import { TAttr } from "@nmide/js-utils";
import { TEvent } from "@nmide/js-utils/lib/TEvent";

document.addEventListener("DOMContentLoaded", () => {
  window.root = document.body;
  listen<TUIInstruction["op"]>("nmide://render", event => {
    parseInst({ op: event.payload });
  }).catch(err => console.error("nmide://render", err));
  invoke<TUIInstruction["op"]>("init").then(op => {
    parseInst({ op });
  }).catch(err => console.error("Init: ", err));
});

const parseInst = ({ op }: TUIInstruction): void => {
  if (op === "noOp") {
    return;
  }
  if ("addAttr" in op) {
    addAttr(document.body, op.addAttr.attr);
    return;
  }
  if ("add" in op) {
    const { ui, id, class: cls } = op.add;
    const html = parseHtml(getHtml(ui));
    if (id !== null) {
      const element = document.getElementById(id);
      if (element !== null) {
        element.appendChild(html);
      }
      return;
    }
    if (cls != null) {
      const elements = document.getElementsByClassName(cls);
      for (let i = 0; i < elements.length; i++) {
        const element = elements[i];
        if (element instanceof HTMLElement) {
          element.appendChild(html)
        }
      }
      return;
    }
    document.body.appendChild(html);
  }
  if ("addAttrPred" in op) {
    const { id, class: cls, attr } = op.addAttrPred;
    if (id !== null) {
      const element = document.getElementById(id);
      if (element !== null) {
        addAttr(element, attr);
      }
      return;
    }
    if (cls != null) {
      const elements = document.getElementsByClassName(cls);
      for (let i = 0; i < elements.length; i++) {
        const element = elements[i];
        if (element instanceof HTMLElement) {
          addAttr(element, attr);
        }
      }
      return;
    }
  }
  if ("textAttrPred" in op) {
    const { id, class: cls, text } = op.textAttrPred;
    if (id !== null) {
      const element = document.getElementById(id);
      if (element !== null) {
        element.innerText = text;
      }
      return;
    }
    if (cls != null) {
      const elements = document.getElementsByClassName(cls);
      for (let i = 0; i < elements.length; i++) {
        const element = elements[i];
        if (element !== null) {
          if (element instanceof HTMLElement) {
            element.innerText = text;
          }
        }
      }
      return;
    }
    document.body.innerText = text;
  }
  if ("then" in op) {
    const { fst, snd } = op.then;
    parseInst({ op: fst });
    parseInst({ op: snd });
    return;
  }
  console.debug("No parse for instruction: ", op);
};

const modifyTHtml = (ui: THtml, f: ((h: THtml) => THtml), p: ((h: THtml) => boolean)): THtml => {
  if (p(ui)) {
    return f(ui);
  }
  return { ...ui, kids: ui.kids.map(k => modifyTHtml(k, f, p)) };
};

const addAttr = (element: HTMLElement, attr: TAttr) => {
  if ("onClick" in attr) {
    element.addEventListener("click", () => onClickParse(attr.onClick))
    return;
  }
  if ("onInput" in attr) {
    return;
  }
  if ("emitInput" in attr) {
    return;
  }
  const attrs = Object.values(attr);
  element.setAttribute(attrs[0], attrs[1]);
}


const onClickParse = (event: TEvent) => {
  return () => {
    invoke("event", { event })
      .catch(err => console.error("Error from onClickParse invoke:", err));
  };
}
