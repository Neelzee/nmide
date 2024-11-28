import { THtml, TMsg } from "@nmide/js-utils";
import { emit } from "@tauri-apps/api/event";


const createElement = ({ kind, kids, text, attrs }: THtml) => {
  const className = attrs.find(el => "class" in el)?.class;
  const id = attrs.find(el => "id" in el)?.id;
  const onClick = attrs.find(el => "onClick" in el)?.onClick;
  const onInput = attrs.find(el => "onInput" in el)?.onInput;
  const src = attrs.find(el => "src" in el)?.src;
  const type = attrs.find(el => "type" in el)?.type;
  const checked = attrs.find(el => "checked" in el)?.checked;

  const elementType = kind === "frag" ? "div" : kind;

  const element = document.createElement(elementType);
  element.textContent = text;
  element.className = className === undefined ? "" : className;
  element.id = id === undefined ? "" : id;
  element.addEventListener("click", OnClickParse(onClick));
  element.addEventListener("onInput", OnInputParse(onInput));
  if (element instanceof HTMLInputElement) {
    element.type = type === undefined ? "" : type;
    element.checked = checked === undefined ? false : checked;
  }
  if (
    element instanceof HTMLScriptElement
    || element instanceof HTMLImageElement
    || element instanceof HTMLAudioElement
    || element instanceof HTMLVideoElement
  ) {
    element.src = src === undefined ? "" : src;
  }

  kids.forEach(kid => element.appendChild(createElement(kid)));

  return element;
}

export const renderHtml = (html: THtml) => {
  const element = parseHtml(html);
  window.root.appendChild(element);
  return element;
}

export const parseHtml = (html: THtml) => {
  // Remove frags
  html.kids.flatMap(kid => kid.kind === "frag" ? kid.kids : [kid]);
  const element = createElement(html);
  return element;
}


function OnClickParse(msg?: TMsg) {
  return () => {
    if (msg === undefined) {
      return;
    }
    emit("msg", msg)
      .catch(err => console.error("Error from OnClickParse emit:", err));
  };
}

function EmitInputParse(msg: string | undefined, value: string) {
  return () => {
    if (msg === undefined) {
      return;
    }
    const tmsg: TMsg = {
      msg: [msg, { str: value }]
    };
    emit("msg", tmsg).catch(err => console.error("Error from EmitInputParse emit:", err));
  };
}

function OnInputParse(msg: TMsg | undefined): () => void {
  return () => {
    if (msg === undefined) {
      return;
    }
    emit("msg", msg).catch(err => console.error("Error from OnInputParse emit:", err));
  };
}
