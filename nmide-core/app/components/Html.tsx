import { Fragment } from "react/jsx-runtime";
import { TSHtml } from "../bindings/TSHtml";
import { v4 as uuidv4 } from "uuid";
import { Msg } from "app/bindings/Msg";
import { TSStyle } from "app/bindings/TSStyle";
import { invoke } from "@tauri-apps/api/core";

export default function RenderHtml({ kind, kids, attrs, text }: TSHtml) {
  const key = uuidv4();
  if (attrs === undefined) {
    attrs = [];
  }
  const txt = text === null ? undefined : text;
  const className = attrs.find(el => "Class" in el)?.Class;
  const id = attrs.find(el => "Id" in el)?.Id;
  const style = attrs.find(el => "Style" in el)?.Style;
  const onClick = attrs.find(el => "OnClick" in el)?.OnClick;
  switch (kind) {
    case "Div":
      return (
        <div
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </div>
      );
    case "P":
      return (
        <p
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </p>
      );
    case "Button":
      return (
        <button
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </button>
      );
    case "Frag":
      return (
        <Fragment
          key={key}
        >
          {txt}
          {kids.map(RenderHtml)}
        </Fragment>
      );
    case "Text":
      return (<>{txt}</>);
    case "H1":
    case "H2":
    case "H3":
    case "H4":
    case "H5":
    case "H6":
    case "Span":
    case "Section":
    case "Article":
    case "Aside":
    case "Audio":
    case "B":
    case "Br":
    case "Code":
    case "Em":
    case "Fieldset":
    case "Form":
    case "Img":
    case "Input":
    case "Label":
    case "Link":
    case "Li":
    case "Menu":
    case "Nav":
    case "Ol":
    case "Option":
    case "Select":
    case "Style":
    case "Svg":
    case "Table":
    case "Td":
    case "Th":
    case "Ul":
    case "Video":
    default:
      return <></>
  }
}

function OnClickParse(msg: Msg | undefined): () => void {
  return () => {
    if (msg === undefined) {
      return;
    }
    invoke("process_msg", { msg: msg as Msg }).catch(err => console.error(err));
  };
}


function RsStyleToReactCSSProperties(style: TSStyle | undefined): React.CSSProperties {
  if (style === undefined) {
    return {};
  }
  return style as React.CSSProperties;
}
