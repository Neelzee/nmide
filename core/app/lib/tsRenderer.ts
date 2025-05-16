import { NmideConfig } from "@nmide/js-core-std-lib";
import {
  Attr,
  Event as NmideEvent,
  Html, HtmlKind,
  Instruction,
  objAdd,
  tStr,
  tValueMaybeOr,
  Value,
  ValueObj
} from "@nmide/js-utils";
import { emit as tauri_emit } from "@tauri-apps/api/event";

const getElementById = (element: HTMLElement, id: string): HTMLElement | undefined => {
  try {
    if (element.matches(`#${id}`)) {
      return element;
    }
  } catch (e) {
    window.__nmideConfig__
      .log
      .error(
        `Exception on selection: ${JSON.stringify(e)}`
        + `, selecting: '#${id}'`
      );
  }
  for (let i = 0; i < element.children.length; i++) {
    const child = element.children[i];
    if (!(child instanceof HTMLElement)) continue;
    const res = getElementById(child, id);
    if (res !== undefined)
      return res;
  }
}

type Emitter = ((event: NmideEvent) => Promise<void>);

export const renderer = (emit: Emitter): NmideConfig["render"] => async (ui) => {
  evalHtml(ui[0], emit);
  evalText(ui[1]);
  evalAttr(ui[2], emit);
}

export const tsRenderer: NmideConfig["render"] = renderer((event: NmideEvent) =>
  tauri_emit("nmide://event", event)
    .catch(err => window.__nmideConfig__.log.error(`Error on emitting: ${JSON.stringify(err)}`)))

const evalHtml = (op: Instruction<Html>, emit: Emitter) => {
  if (op === "noOp" || op === null || op === undefined) {
    return;
  }
  if ("add" in op) {
    const id = op.add[0];
    const ui = op.add[1];
    const thtml = getHtml(ui);
    const html = parseHtml(thtml, emit);
    if (id !== "") {
      const element = getElementById(window.__nmideConfig__.root, id);
      if (element !== undefined) {
        element.appendChild(html);
      } else {
        window.__nmideConfig__.log.error(`Could not find HTMLElement with id: "${id}"`);
      }
      return;
    }
    window.__nmideConfig__.root.appendChild(html);
    return;
  }
  if ("then" in op) {
    const fst = op.then[0];
    const snd = op.then[1];
    evalHtml(fst, emit);
    evalHtml(snd, emit);
    return;
  }
  window.__nmideConfig__.log.debug(`No parse for html-instruction: ${JSON.stringify(op)}`);
}

const evalAttr = (op: Instruction<Attr>, emit: Emitter) => {
  if (op === "noOp" || op === null || op === undefined) {
    return;
  }
  if ("add" in op) {
    const id = op.add[0];
    const attr = op.add[1];
    if (id !== null && id !== "") {
      const element = getElementById(window.__nmideConfig__.root, id);
      if (element !== undefined) {
        addAttr(element, attr, emit);
      } else {
        window.__nmideConfig__.log.error(`Could not find HTMLElement with id: "${id}"`);
      }
      return;
    }
  }
  if ("rem" in op) {
    const id = op.rem[0];
    const attr = op.rem[1];
    if (id !== null && id !== "") {
      const element = getElementById(window.__nmideConfig__.root, id);
      if (element !== undefined) {
        remAttr(element, attr);
      } else {
        window.__nmideConfig__.log.error(`Could not find HTMLElement with id: "${id}"`);
      }
      return;
    }
  }
  if ("then" in op) {
    const fst = op.then[0];
    const snd = op.then[1];
    evalAttr(fst, emit);
    evalAttr(snd, emit);
    return;
  }
  window.__nmideConfig__.log.debug(`No parse for attr-instruction: ${JSON.stringify(op)}`);
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
      } else {
        window.__nmideConfig__.log.error(`Could not find HTMLElement with id: "${id}"`);
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
      } else {
        window.__nmideConfig__.log.error(`Could not find HTMLElement with id: "${id}"`);
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
  window.__nmideConfig__.log.debug(`No parse for text-instruction: ${JSON.stringify(op)}`);
}

const remAttr = (element: HTMLElement, attr: Attr) => {
  if ("click" in attr) {
    return;
  }
  if ("onInput" in attr) {
    return;
  }
  if ("emitInput" in attr) {
    return;
  }

  const attrs = Object.entries(attr);
  const [a, val] = attrs[0];
  const att = a === "clss" ? "class" : a;
  if (val === "") {
    element.removeAttribute(att);
  } else {
    for (const attr of element.attributes) {
      if (attr.name === att) {
        const oldValue = attr.value;
        attr.value = oldValue.split(" ").filter(v => {
          return v !== val;
        }).join(" ");
      }
    }
  }
}

const addAttr = (element: HTMLElement, attr: Attr, emit: Emitter) => {
  if ("click" in attr) {
    element.addEventListener(
      "click",
      function (this, ev: MouseEvent) {
        clickParse(attr.click, this, ev, emit)();
      }
    );
    return;
  }
  if ("change" in attr) {
    element.addEventListener(
      "change",
      function (this, ev: Event) {
        changeParse(attr.change, this, ev, emit)();
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
  if ("clss" in attr) {
    const value = element.getAttribute("class");
    element.setAttribute("class", `${value === null ? "" : value} ${attr.clss}`);
    return;
  }
  if ("custom" in attr) {
    const [k, v] = attr.custom;
    element.setAttribute(k, v);
    return;
  }
  window.__nmideConfig__.log.error(`No Attribute: ${attr}`);
};

const formHandler = (args: ValueObj, form: HTMLFormElement): ValueObj => {
  const data = new FormData(form);
  const obj: Record<string, Value> = {};
  for (const [k, v] of data.entries()) {
    obj[k] = tValueMaybeOr(v)(tStr(v.toString()));
  }
  return objAdd(args, "form", { obj });
}

const changeParse = (event: NmideEvent, ts: HTMLElement, evt: Event, emit: Emitter) => {
  evt.preventDefault();
  let args: ValueObj = { obj: {} };

  if (evt.target !== null && evt.target instanceof Element) {
    const form = evt.target.closest("form");
    if (form !== null) {
      args = formHandler(args, form);
    }
  }

  if (ts instanceof HTMLSelectElement) {
    const val = ts.value;
    const id = ts.getAttribute("id");
    const name = ts.getAttribute("name");
    if (name !== null) {
      args = objAdd(args, name, tStr(val));
    } else if (id !== null) {
      args = objAdd(args, id, tStr(val));
    }
  }

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
      args = objAdd(args, "lineNumber", { int: ln });
      args = objAdd(args, "columnNumber", { int: cn });
    }
  }
  args = objAdd(args, "id", { str: ts.id });
  if (typeof event === "object") {
    if ("event" in event) {
      if (event.event.args !== null) {
        args = objAdd(args, "eventArgs", event.event.args);
      }
      event = { event: { event: event.event.event, args } };
    }
  }

  return () => {
    emit(event).catch((err) =>
      window.__nmideConfig__.log.error("Error from onClickParse invoke:", err),
    );
  };
};

const clickParse = (event: NmideEvent, ts: HTMLElement, evt: MouseEvent, emit: Emitter) => {
  evt.preventDefault();
  let args: ValueObj = { obj: {} };

  if (evt.target !== null && evt.target instanceof Element) {
    const form = evt.target.closest("form");
    if (form !== null) {
      args = formHandler(args, form);
    }
  }

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
      args = objAdd(args, "lineNumber", { int: ln });
      args = objAdd(args, "columnNumber", { int: cn });
    }
  }
  args = objAdd(args, "id", { str: ts.id });
  if (typeof event === "object") {
    if ("event" in event) {
      if (event.event.args !== null) {
        args = objAdd(args, "eventArgs", event.event.args);
      }
      event = { event: { event: event.event.event, args } };
    }
  }

  return () => {
    emit(event).catch((err) =>
      window.__nmideConfig__.log.error("Error from onClickParse invoke:", err),
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
const createElement = ({ kind, attrs, kids, text }: THtml, emit: Emitter) => {
  const className = attrs.find((el) => "clss" in el)?.clss;
  const id = attrs.find((el) => "id" in el)?.id;
  const onClick = attrs.find((el) => "click" in el)?.click;
  const change = attrs.find((el) => "change" in el)?.change;
  //const onInput = attrs.find(el => "onInput" in el)?.onInput;
  const src = attrs.find((el) => "src" in el)?.src;
  const type = attrs.find((el) => "type" in el)?.type;
  const checked = attrs.find((el) => "checked" in el)?.checked;
  const custom = attrs.find(el => "custom" in el)?.custom;

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
        clickParse(onClick, this, ev, emit)();
      }
    );
  if (change !== undefined)
    element.addEventListener(
      "change",
      function (this, ev: Event) {
        changeParse(change, this, ev, emit)();
      }
    );
  if (custom !== undefined) {
    const [k, v] = custom;
    element.setAttribute(k, v);
  }
  //if (onInput !== undefined)
  //element.addEventListener("onInput", OnInputParse(onInput));
  if (element instanceof HTMLButtonElement) {
    switch (type) {
      case "button":
      case "reset":
      case "submit":
        element.type = type;
        break;
      default:
        break;
    }
  }
  if (element instanceof HTMLInputElement) {
    if (type !== undefined) element.type = type;
    if (checked !== undefined) element.checked = checked;
    if (custom !== undefined) {
      const [k, v] = custom;
      switch (k) {
        case "disabled":
          element.disabled = v === "true";
          break;
        default:
          break;
      }
    }
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

  kids.forEach((kid) => element.appendChild(createElement(kid, emit)));

  return element;
};

// TODO: Add docs
export const parseHtml = (html: THtml, emit: Emitter) => {
  return createElement(html, emit);
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
