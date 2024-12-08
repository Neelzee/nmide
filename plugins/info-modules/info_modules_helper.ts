import { emptyHtml, TMap, TMsg } from "@nmide/js-utils";
import { getCurrentWebview } from "@tauri-apps/api/webview";

let filePath: string = null;

let init = false;

getCurrentWebview().listen<void>("nmide://view", () => {
  if (init) return;
  const element = document.getElementById("info_modules_helper_id");
  init = true;
  if (element === null) return;
  element.addEventListener("click", _ => {
    filePath = window.prompt("Insert filepath to render");
    if (filePath !== null) {
      getCurrentWebview().emitTo(
        "main",
        "msg",
        { msg: ["info-module-find-file", { str: filePath }] }
      );
    }
  });
});

window.plugins.set(
  "info_modules_helper",
  {
    init: () => {
      window.plugins.get("ide-view").parseHtml({
        kind: "button",
        attrs: [
          { id: "info_modules_helper_id" },
          { class: "location-tool-bar" },
        ],
        kids: [],
        text: "Dependency Graph",
      });
      return [];
    },
    view: (_: TMap) => emptyHtml(),
    update: (_: TMsg, __: TMap) => {
      return [];
    }
  }
);
