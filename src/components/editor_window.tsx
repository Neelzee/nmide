import React from "react";
import { EditorTab } from "./editor_tab";

export class EditorWindow extends React.Component {
  render() {
    return (
      <section className="editor">
        <EditorTab />
      </section>
    );
  }
}
