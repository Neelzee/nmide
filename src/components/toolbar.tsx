import { Setter } from "solid-js";
import "../styles/toolbar.scss";
import { Folder } from "../types.ts";

export function ToolBar(props: { setFiles: Setter<Folder> }) {
  return (
    <section id="toolbar">
      <span class="toolbar-elem logo">
        Logo
      </span>
      <span class="toolbar-elem">
        File
      </span>
      <span class="toolbar-elem">
        Edit
      </span>
      <span class="toolbar-elem">
        Selection
      </span>
      <span class="toolbar-elem">
        View
      </span>
      <span class="toolbar-elem">
        Go
      </span>
      <span class="toolbar-elem">
        Run
      </span>
      <span class="toolbar-elem">
        Terminal
      </span>
      <span class="toolbar-elem">
        Help
      </span>
    </section>
  );
}
