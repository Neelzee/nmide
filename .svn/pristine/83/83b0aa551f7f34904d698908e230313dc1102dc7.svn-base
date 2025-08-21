import "@nmide/js-utils";
import { emptyHtml, TMap, TMsg } from "@nmide/js-utils";
import { editor } from "monaco-editor";

let init = false;
window.plugins.set(
  "ide-editor",
  {
    init: () => [],
    view: (_: TMap) => {
      if (init) return [];
      const root = document.getElementById("root");
      if (root === null) return [];
      const container = document.createElement("div");
      container.id = "ide_container";
      root.appendChild(container);
      init = true;
      const _editor = editor.create(container, {
        // Text in the editor
        value: `function hello() {
  alert("Hello, World!);
}`,
        language: "javascript",
        automaticLayout: true,
      });
      return emptyHtml();
    },
    update: (_: TMsg, __: TMap) => [],
  }
)
