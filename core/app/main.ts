import { App } from "./App";
import { parseHtml, renderHtml } from "./lib/renderHtml";

window.renderHtml = renderHtml;
window.parseHtml = parseHtml;
window.root = document.body;
window.addEventListener("DOMContentLoaded", () => App());
