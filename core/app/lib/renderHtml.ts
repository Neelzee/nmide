import { TAttr } from "@nmide/js-utils";
import { Html } from "@nmide/js-utils/lib/Html";
import { TEvent } from "@nmide/js-utils/lib/TEvent";
import { invoke } from "@tauri-apps/api/core";

type THtml = { kind: string, kids: THtml[], attrs: TAttr[], text?: string };

// TODO: Add docs
const createElement = ({ attrs, kids, kind, text }: THtml) => {
  const className = attrs.find(el => "class" in el)?.class;
  const id = attrs.find(el => "id" in el)?.id;
  const onClick = attrs.find(el => "onClick" in el)?.onClick;
  //const onInput = attrs.find(el => "onInput" in el)?.onInput;
  const src = attrs.find(el => "src" in el)?.src;
  const type = attrs.find(el => "type" in el)?.type;
  const checked = attrs.find(el => "checked" in el)?.checked;

  const elementType = kind === "frag" ? "div" : kind;

  const element = document.createElement(elementType);
  element.textContent = text ? text : null;
  if (className !== undefined) element.className = className;
  if (id !== undefined) element.id = id;
  if (onClick !== undefined)
    element.addEventListener("click", OnClickParse(onClick));
  //if (onInput !== undefined)
  //element.addEventListener("onInput", OnInputParse(onInput));
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

// TODO: Add docs
export const renderHtml = (h: Html) => {
  //const element = window.parseHtml(html);
  const html = getHtml(h);
  const element = parseHtml(html);
  // HACK: The reason this is undefined, is because rendering an
  // Html-frag-node should not happen, so this has to be encoded with undefined
  if (html.kind === "frag" && html.kids.length === 0 || element === undefined) return;
  window.root.appendChild(element);
  return element;
}

// TODO: Add docs
export const parseHtml = (html: THtml) => {
  // HACK: A lot of plugins use "frag" to mean an empty HTML node, this was/is
  // rendered as a `div`. The preferred behaviour is to not render it at all,
  // and _unpack_ the kids of the node. This is not done.
  // The reason a lot of plugins do this, is because they might not have any
  // Html they want to render.
  html.kids.flatMap(kid => kid.kind === "frag" ? kid.kids : [kid]);
  const element = createElement(html);
  return element;
}


// TODO: Add docs
function OnClickParse(event: TEvent) {
  return () => {
    console.log("Clicking")
    invoke("event", { event })
      .catch(err => console.error("Error from OnClickParse emit:", err));
  };
}

export const getHtml = (html: Html): THtml => {
  const elements = ["p", "main", "button", "div"];
  for (let index = 0; index < elements.length; index++) {
    const element = elements[index];
    //@ts-ignore
    if (element in html) return { kind: element, ...html[element], kids: html[element].kids.map(getHtml) };
  }
  //@ts-ignore
  return { kind: "p", text: html?.text, kids: [], attrs: [] };
};
