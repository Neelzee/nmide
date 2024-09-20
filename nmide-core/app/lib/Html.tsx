import { Fragment } from "react/jsx-runtime";
import { v4 as uuidv4 } from "uuid";
import { invoke } from "@tauri-apps/api/core";
import { THtml } from "../lib/bindings/THtml";
import React from "react";
import { TMsg } from "./bindings/TMsg";

export default function RenderHtml({ kind, kids, attrs, text }: THtml) {
  const key = uuidv4();
  const txt = text === null ? undefined : text;
  const className = attrs.find(el => "Class" in el)?.Class;
  const id = attrs.find(el => "Id" in el)?.Id;
  const style = undefined;
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

function OnClickParse(msg: TMsg | undefined): () => void {
  return () => {
    if (msg === undefined) {
      return;
    }
    invoke("process_msg", { msg: msg as TMsg }).catch(err => console.error(err));
  };
}


function RsStyleToReactCSSProperties(_: undefined): React.CSSProperties | undefined {
  return undefined;
}
