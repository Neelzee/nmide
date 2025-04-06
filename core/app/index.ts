import { invoke } from "@tauri-apps/api/core";


document.addEventListener("DOMContentLoaded", () => {
  invoke("init").then(v => console.log(v));
})
