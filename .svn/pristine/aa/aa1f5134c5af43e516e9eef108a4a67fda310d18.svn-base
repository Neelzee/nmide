import { THtml, TMsg } from "@nmide/js-utils";

// TODO: Add docs
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

// TODO: Add docs
export const renderHtml = (html: THtml) => {
  const element = window.parseHtml(html);
  // HACK: The reason this is undefined, is because rendering an
  // THtml-frag-node should not happen, so this has to be encoded with undefined
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
  // THtml they want to render.
  html.kids.flatMap(kid => kid.kind === "frag" ? kid.kids : [kid]);
  const element = createElement(html);
  return element;
}


// TODO: Add docs
function OnClickParse(msg: TMsg) {
  return () => {
    window.emit("msg", msg)
      .catch(err => window.log.error("Error from OnClickParse emit:", err));
  };
}

// TODO: Add docs
function OnInputParse(msg: TMsg): () => void {
  return () => {
    window.emit("msg", msg)
      .catch(err => window.log.error("Error from OnInputParse emit:", err));
  };
}
