import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { renderHtml } from "./lib/renderHtml";
import { Html } from "@nmide/js-utils/lib/Html";

document.addEventListener("DOMContentLoaded", () => {
  window.root = document.body;
  listen("counter", event => {
    console.log(event.payload);
  });
  listen<Html>("nmide://render", event => {
    console.log(event);
    renderHtml(event.payload);
  });
  invoke<Html>("init").then(evt => {
    renderHtml(evt);
  });
})
