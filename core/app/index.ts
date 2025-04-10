import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Html, TUIInstruction } from "@nmide/js-utils/lib/Html";
import { TAttr } from "@nmide/js-utils";
import { TEvent } from "@nmide/js-utils/lib/TEvent";

export type THtml = {
  kind: string;
  kids: THtml[];
  attrs: TAttr[];
  text?: string;
};

document.addEventListener("DOMContentLoaded", () => {
  window.root = document.body;
  listen<TUIInstruction["op"]>("nmide://render", (event) => {
    parseInst({ op: event.payload });
  }).catch((err) => console.error("nmide://render", err));
  invoke<TUIInstruction["op"]>("init")
    .then((op) => {
      parseInst({ op });
    })
    .catch((err) => console.error("Init: ", err));
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
          element.appendChild(html);
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

const addAttr = (element: HTMLElement, attr: TAttr) => {
  if ("onClick" in attr) {
    element.addEventListener("click", () => onClickParse(attr.onClick));
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

const onClickParse = (event: TEvent) => {
  return () => {
    invoke("event", { event }).catch((err) =>
      console.error("Error from onClickParse invoke:", err),
    );
  };
};

// TODO: Add docs
const createElement = ({ attrs, kids, kind, text }: THtml) => {
  const className = attrs.find((el) => "class" in el)?.class;
  const id = attrs.find((el) => "id" in el)?.id;
  const onClick = attrs.find((el) => "onClick" in el)?.onClick;
  //const onInput = attrs.find(el => "onInput" in el)?.onInput;
  const src = attrs.find((el) => "src" in el)?.src;
  const type = attrs.find((el) => "type" in el)?.type;
  const checked = attrs.find((el) => "checked" in el)?.checked;

  const elementType = kind === "frag" ? "div" : kind;

  const element = document.createElement(elementType);
  element.textContent = text ? text : null;
  if (className !== undefined) element.className = className;
  if (id !== undefined) element.id = id;
  if (onClick !== undefined)
    element.addEventListener("click", onClickParse(onClick));
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
  // HACK: A lot of plugins use "frag" to mean an empty HTML node, this was/is
  // rendered as a `div`. The preferred behaviour is to not render it at all,
  // and _unpack_ the kids of the node. This is not done.
  // The reason a lot of plugins do this, is because they might not have any
  // Html they want to render.
  html.kids.flatMap((kid) => (kid.kind === "frag" ? kid.kids : [kid]));
  return createElement(html);
};

export const getHtml = (html: Html): THtml => {
  const kind = Object.keys(html).find(
    (k) => k !== "kids" && k !== "attrs" && k !== "text",
  );
  return {
    //@ts-expect-error This is valid as long as `Html` doesn't change
    kind,
    ...Object.keys(html)
      .filter((f) => f !== kind)
      //@ts-expect-error This will always be valid
      .map((f) => html[f]),
  };
};
