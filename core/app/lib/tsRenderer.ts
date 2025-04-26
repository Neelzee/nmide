import {
  Attr,
  Event,
  Html, HtmlKind,
  Instruction,
  ValueObj
} from "@nmide/js-utils";
import { emit } from "@tauri-apps/api/event";

const getElementById = (element: HTMLElement, id: string): HTMLElement | undefined => {
  if (element.matches(`#${id}`)) {
    return element;
  }
  for (let i = 0; i < element.children.length; i++) {
    const child = element.children[i];
    if (!(child instanceof HTMLElement)) continue;
    const res = getElementById(child, id);
    if (res !== undefined)
      return res;
  }
}

export const tsRenderer = async (ui: [Instruction<Html>, Instruction<string>, Instruction<Attr>]) => {
  evalHtml(ui[0]);
  evalText(ui[1]);
  evalAttr(ui[2]);
}

const evalHtml = (op: Instruction<Html>) => {
  if (op === "noOp" || op === null || op === undefined) {
    return;
  }
  if ("add" in op) {
    const id = op.add[0];
    const ui = op.add[1];
    const html = parseHtml(getHtml(ui));
    if (id !== null && id !== "") {
      const element = getElementById(window.__nmideConfig__.root, id);
      if (element !== undefined) {
        element.appendChild(html);
      }
      return;
    }
    window.__nmideConfig__.root.appendChild(html);
    return;
  }
  if ("then" in op) {
    const fst = op.then[0];
    const snd = op.then[1];
    evalHtml(fst);
    evalHtml(snd);
    return;
  }
  window.__nmideConfig__.log.debug("No parse for instruction: ", op);
}

const evalAttr = (op: Instruction<Attr>) => {
  if (op === "noOp" || op === null || op === undefined) {
    return;
  }
  if ("add" in op) {
    const id = op.add[0];
    const attr = op.add[1];
    if (id !== null && id !== "") {
      const element = getElementById(window.__nmideConfig__.root, id);
      if (element !== undefined) {
        addAttr(element, attr);
      }
      return;
    }
  }
  if ("then" in op) {
    const fst = op.then[0];
    const snd = op.then[1];
    evalAttr(fst);
    evalAttr(snd);
    return;
  }
  window.__nmideConfig__.log.debug("No parse for instruction: ", op);
}

const evalText = (op: Instruction<string>) => {
  if (op === "noOp" || op === null || op === undefined) {
    return;
  }
  if ("rem" in op) {
    const id = op.rem[0];
    const cls = op.rem[1];
    if (id !== null) {
      const element = getElementById(window.__nmideConfig__.root, id);
      if (element !== undefined) {
        element.innerText = "";
      }
      return;
    }
    if (cls != null) {
      const elements = window.__nmideConfig__.root.getElementsByClassName(cls);
      for (let i = 0; i < elements.length; i++) {
        const element = elements[i];
        if (element !== null) {
          if (element instanceof HTMLElement) {
            element.innerText = "";
          }
        }
      }
      return;
    }
  }
  if ("add" in op) {
    const id = op.add[0];
    const text = op.add[1];
    if (id !== null && text !== "") {
      const element = getElementById(window.__nmideConfig__.root, id);
      if (element !== undefined) {
        element.innerText = text;
      }
      return;
    }
  }
  if ("then" in op) {
    const fst = op.then[0];
    const snd = op.then[1];
    evalText(fst);
    evalText(snd);
    return;
  }
  window.__nmideConfig__.log.debug("No parse for instruction: ", op);
}

const addAttr = (element: HTMLElement, attr: Attr) => {
  if ("click" in attr) {
    element.addEventListener(
      "click",
      function (this, ev: MouseEvent) {
        clickParse(attr.click, this, ev)();
      }
    );
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
};

const clickParse = (event: Event, ts: HTMLElement, _: MouseEvent) => {
  let args: ValueObj = { obj: { } };
  if (ts instanceof HTMLInputElement || ts.tagName === "TEXTAREA") {
    // @ts-expect-error selectionStart exists on ts
    if (ts.selectionStart !== null) {
      // @ts-expect-error selectionStart exists on ts
      const pos: number = ts.selectionStart;
      // @ts-expect-error value exists on ts
      const txt: string = ts.value;
      let ln = 1;
      let cn = 1;
      for (let i = 0; i < pos; i++) {
        if (txt[i] === "\n") {
          ln++;
          cn = 1;
        } else {
          cn++;
        }
      }
      args = {
        obj: {
          lineNumber: { int: ln },
          columnNumber: { int: cn },
        }
      };
    }
  }
  args = { obj: { ...args.obj,  id: { str: ts.id }, } };
  if (event.args !== null) {
    args = { obj: { ...args.obj, eventArgs: event.args } };
  }

  return () => {
    emit("nmide://event", { ...event, args }).catch((err) =>
      console.error("Error from onClickParse invoke:", err),
    );
  };
};

type THtml = {
  kind: HtmlKind,
  kids: THtml[],
  attrs: Attr[],
  text: string | null,
}

// TODO: Add docs
const createElement = ({ kind, attrs, kids, text }: THtml) => {
  const className = attrs.find((el) => "class" in el)?.class;
  const id = attrs.find((el) => "id" in el)?.id;
  const onClick = attrs.find((el) => "click" in el)?.click;
  //const onInput = attrs.find(el => "onInput" in el)?.onInput;
  const src = attrs.find((el) => "src" in el)?.src;
  const type = attrs.find((el) => "type" in el)?.type;
  const checked = attrs.find((el) => "checked" in el)?.checked;

  const elementType = kind === "frag" ? "div" : kind;

  // @ts-expect-error elementType is a string type
  const element: HTMLElement = document.createElement(elementType);
  element.textContent = text ? text : null;
  if (className !== undefined) element.className = className;
  if (id !== undefined) element.id = id;
  if (onClick !== undefined)
    element.addEventListener(
      "click",
      function (this, ev: MouseEvent) {
        clickParse(onClick, this, ev)();
      }
    );
  //if (onInput !== undefined)
  //element.addEventListener("onInput", OnInputParse(onInput));
  if (element instanceof HTMLInputElement) {
    if (type !== undefined) element.type = type;
    if (checked !== undefined) element.checked = checked;
  }
  if (
    (element instanceof HTMLScriptElement ||
      element instanceof HTMLImageElement ||
      element instanceof HTMLAudioElement ||
      element instanceof HTMLVideoElement) &&
    src !== undefined
  ) {
    element.src = src;
  }

  kids.forEach((kid) => element.appendChild(createElement(kid)));

  return element;
};

// TODO: Add docs
export const parseHtml = (html: THtml) => {
  return createElement(html);
};

export const getHtml = (html: Html): THtml => {
  const kind = Object.keys(html)[0];
  return {
    kind,
    //@ts-expect-error This is valid as long as `Html` doesn't change
    ...html[kind],
    //@ts-expect-error This is valid as long as `Html` doesn't change
    kids: html[kind].kids.map(getHtml)
  };
};
