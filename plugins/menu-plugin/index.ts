import { getCurrentWebview } from "@tauri-apps/api/webview";

const webview = getCurrentWebview();

const input = document.getElementById("menu-path");
const btn = document.getElementById("menu-btn");

if (btn !== null && input !== null && input instanceof HTMLInputElement) {
  btn.addEventListener("click", () => {
    webview.emit(
      "setDocument",
      { field: "workplace-path", value: input.value }
    ).then(_ => _).catch(err => console.error(err));
  });
}
