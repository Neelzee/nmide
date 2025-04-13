import { Html, TUIInstruction } from "@nmide/js-utils/lib/Html";
import { TAttr, TEvent, THtml, TValue } from "@nmide/js-utils";
import { invoke } from "@tauri-apps/api/core";
import { Instruction } from "@nmide/js-utils/lib/Instruction.ts";

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

export const tsRenderer = async (op: TUIInstruction) => {
  const { ui } = op;
  for (let i = 0; i < ui[0].length + ui[1].length + ui[2].length; i++) {
    const html = ui[0].find(([j, _]) => j == i);
    if (html !== undefined) {
      evalHtml(html[1]);
      continue;
    }

    const text = ui[1].find(([j, _]) => j == i);
    if (text !== undefined) {
      evalText(text[1]);
      continue;
    }

    const attr = ui[2].find(([j, _]) => j == i);
    if (attr !== undefined) {
      evalAttr(attr[1]);
    }
  }
}

const evalState = async (op: Instruction<TValue>) => {
  if (op === "noOp" || op === null || op === undefined) {
    return;
  }
  if ("add" in op) {

  }
}

const evalHtml = (op: Instruction<Html>) => {
  if (op === "noOp" || op === null || op === undefined) {
    return;
  }
  if ("add" in op) {
    const id = op.add[0];
    const cls = op.add[1];
    const ui = op.add[2];
    const html = parseHtml(getHtml(ui));
    if (id !== null) {
      const element = getElementById(window.__nmideConfig__.root, id);
      if (element !== undefined) {
        element.appendChild(html);
      }
      return;
    }
    if (cls != null) {
      const elements = window.__nmideConfig__.root.getElementsByClassName(cls);
      for (let i = 0; i < elements.length; i++) {
        const element = elements[i];
        if (element instanceof HTMLElement) {
          element.appendChild(html);
        }
      }
      return;
    }
    window.__nmideConfig__.root.appendChild(html);
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

const evalAttr = (op: Instruction<TAttr>) => {
  if (op === "noOp" || op === null || op === undefined) {
    return;
  }
  if ("add" in op) {
    const id = op.add[0];
    const cls = op.add[1];
    const attr = op.add[2];
    if (id !== null) {
      const element = getElementById(window.__nmideConfig__.root, id);
      if (element !== undefined) {
        addAttr(element, attr);
      }
      return;
    }
    if (cls != null) {
      const elements = window.__nmideConfig__.root.getElementsByClassName(cls);
      for (let i = 0; i < elements.length; i++) {
        const element = elements[i];
        if (element instanceof HTMLElement) {
          addAttr(element, attr);
        }
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
    const cls = op.add[1];
    const text = op.add[2];
    if (id !== null) {
      const element = getElementById(window.__nmideConfig__.root, id);
      if (element !== undefined) {
        element.innerText = text;
      }
      return;
    }
    if (cls != null) {
      const elements = window.__nmideConfig__.root.getElementsByClassName(cls);
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
  html.kids = html.kids.flatMap((kid) => (kid.kind === "frag" ? kid.kids : [kid]));
  return createElement(html);
};

export const getHtml = (html: Html): THtml => {
  const kind = Object.keys(html).find(
    (k) => k !== "kids" && k !== "attrs" && k !== "text",
  );
  return {
    kind,
    //@ts-expect-error This is valid as long as `Html` doesn't change
    ...html[kind],
    //@ts-expect-error This is valid as long as `Html` doesn't change
    kids: html[kind].kids.map(getHtml)
  };
};
