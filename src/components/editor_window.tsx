import React from "react";
import { EditorTab } from "./editor_tab";
import { CodeEditor } from "./code_editor";

export class EditorWindow extends React.Component {
  render() {
    return (
      <section className="editor">
        <EditorTab />
        <CodeEditor />
      </section>
    );
  }
}
