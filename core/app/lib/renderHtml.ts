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
  if (className !== undefined) element.className = className;
  if (id !== undefined) element.id = id;
  if (onClick !== undefined)
    element.addEventListener("click", OnClickParse(onClick));
  if (onInput !== undefined)
    element.addEventListener("onInput", OnInputParse(onInput));
  if (element instanceof HTMLInputElement) {
    if (type !== undefined) element.type = type;
    if (checked !== undefined) element.checked = checked;
  }
  if (
    (element instanceof HTMLScriptElement
      || element instanceof HTMLImageElement
      || element instanceof HTMLAudioElement
      || element instanceof HTMLVideoElement)
    && src !== undefined
  ) {
    element.src = src;
  }

  kids.forEach(kid => element.appendChild(createElement(kid)));

  return element;
}

export const renderHtml = (html: THtml) => {
  const element = parseHtml(html);
  if (html.kind === "frag" && html.kids.length === 0) return;
  window.root.appendChild(element);
  return element;
}

export const parseHtml = (html: THtml) => {
  // Remove frags
  html.kids.flatMap(kid => kid.kind === "frag" ? kid.kids : [kid]);
  const element = createElement(html);
  return element;
}


function OnClickParse(msg: TMsg) {
  return () => {
    emit("msg", msg)
      .catch(err => console.error("Error from OnClickParse emit:", err));
  };
}

function OnInputParse(msg: TMsg): () => void {
  return () => {
    emit("msg", msg)
      .catch(err => console.error("Error from OnInputParse emit:", err));
  };
}
