import { invoke } from "@tauri-apps/api/core";
import { emit, listen } from "@tauri-apps/api/event";


document.addEventListener("DOMContentLoaded", () => {
  invoke("init").then(v => console.log(v));
})
