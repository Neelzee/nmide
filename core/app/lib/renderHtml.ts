import { THtml, TMsg } from "@nmide/js-utils";
import { emit } from "@tauri-apps/api/event";


export const root = document.getElementById("root");

if (root == null) throw Error("Root is null");

const createElement = ({ kind, kids, text, attrs }: THtml) => {
  const className = attrs.find(el => "Class" in el)?.Class;
  const id = attrs.find(el => "Id" in el)?.Id;
  const onClick = attrs.find(el => "OnClick" in el)?.OnClick;
  const onInput = attrs.find(el => "OnInput" in el)?.OnInput;
  const src = attrs.find(el => "Src" in el)?.Src;
  const type = attrs.find(el => "Type" in el)?.Type;
  const checked = attrs.find(el => "Checked" in el)?.Checked;

  const elementType = kind === "Frag" ? "div" : kind.toLowerCase();

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
  // Remove frags
  html.kids.flatMap(kid => kid.kind === "Frag" ? kid.kids : [kid]);

  const element = createElement(html);

  root.appendChild(element);

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
      Msg: [msg, { Str: value }]
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
