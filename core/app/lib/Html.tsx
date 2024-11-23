import { Fragment } from "react/jsx-runtime";
import { v4 as uuidv4 } from "uuid";
import { THtml } from "@nmide/js-utils";
import React from "react";
import { TMsg } from "@nmide/js-utils";
import { emit } from "@tauri-apps/api/event";

export default function RenderHtml({ kind, kids, attrs, text }: THtml) {
  const key = uuidv4();
  const txt = text === null ? undefined : text;
  const className = attrs.find(el => "Class" in el)?.Class;
  const id = attrs.find(el => "Id" in el)?.Id;
  const style = undefined;
  const onClick = attrs.find(el => "OnClick" in el)?.OnClick;
  const onInput = attrs.find(el => "OnInput" in el)?.OnInput;
  const emitInput = attrs.find(el => "EmitInput" in el)?.EmitInput;
  const src = attrs.find(el => "Src" in el)?.Src;
  const type = attrs.find(el => "Type" in el)?.Type;
  const checked = attrs.find(el => "Checked" in el)?.Checked;
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
      return (
        <h1
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </h1>
      );
    case "H2":
      return (
        <h2
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </h2>
      );
    case "H3":
      return (
        <h3
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </h3>
      );
    case "H4":
      return (
        <h4
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </h4>
      );
    case "H5":
      return (
        <h5
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </h5>
      );
    case "H6":
      return (
        <h6
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </h6>
      );
    case "Span":
      return (
        <span
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </span>
      );
    case "Section":
      return (
        <section
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </section>
      );
    case "Article":
      return (
        <article
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </article>
      );
    case "Aside":
      return (
        <aside
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </aside>
      );
    case "Audio":
      return (
        <audio
          key={key}
          className={className}
          id={id}
          src={src}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </audio>
      );
    case "B":
      return (
        <b
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </b>
      );
    case "Br":
      return (
        <br
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </br>
      );
    case "Code":
      return (
        <code
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </code>
      );
    case "Em":
      return (
        <em
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </em>
      );
    case "Fieldset":
      return (
        <fieldset
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </fieldset>
      );
    case "Form":
      return (
        <form
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </form>
      );
    case "Img":
      return (
        <img
          key={key}
          className={className}
          id={id}
          src={src}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </img>
      );
    case "Input":
      return (
        <>
          <input
            key={key}
            className={className}
            id={id}
            style={RsStyleToReactCSSProperties(style)}
            onClick={OnClickParse(onClick)}
            onInput={OnInputParse(onInput)}
            onChange={el => {
              EmitInputParse(emitInput, el.target.value)
            }}
            type={type}
            checked={checked}
          >
          </input>

          {txt}
          {kids.map(RenderHtml)}
        </>
      );
    case "Label":
      return (
        <label
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </label>
      );
    case "Link":
      return (
        <link
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </link>
      );
    case "Li":
      return (
        <li
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </li>
      );
    case "Menu":
      return (
        <menu
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </menu>
      );
    case "Nav":
      return (
        <nav
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </nav>
      );
    case "Ol":
      return (
        <ol
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </ol>
      );
    case "Option":
      return (
        <option
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </option>
      );
    case "Select":
      return (
        <select
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </select>
      );
    case "Style":
      return (
        <style
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </style>
      );
    case "Svg":
      return (
        <svg
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </svg>
      );
    case "Table":
      return (
        <table
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </table>
      );
    case "Tbody":
      return (
        <tbody
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </tbody>
      );
    case "Td":
      return (
        <td
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </td>
      );
    case "Th":
      return (
        <th
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </th>
      );
    case "Tr":
      return (
        <tr
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </tr>
      );
    case "Ul":
      return (
        <ul
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
        >
          {txt}
          {kids.map(RenderHtml)}
        </ul>
      );
    case "Video":
      return (
        <video
          key={key}
          className={className}
          id={id}
          style={RsStyleToReactCSSProperties(style)}
          onClick={OnClickParse(onClick)}
          src={src}
        >
          {txt}
          {kids.map(RenderHtml)}
        </video>
      );
    default:
      return <></>
  }
}

function OnClickParse(msg: TMsg | undefined): () => void {
  return () => {
    if (msg === undefined) {
      return;
    }
    emit("msg", msg).catch(err => console.error("Error from OnClickParse emit:", err));
  };
}

function EmitInputParse(msg: string | undefined, value: string): React.ChangeEventHandler<HTMLInputElement> | undefined {
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

function RsStyleToReactCSSProperties(_: undefined): React.CSSProperties | undefined {
  return undefined;
}
