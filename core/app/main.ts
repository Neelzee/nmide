import { App } from "./App";
import { parseHtml, renderHtml } from "./lib/renderHtml";

window.renderHtml = renderHtml;
window.parseHtml = parseHtml;
window.addEventListener("DOMContentLoaded", () => App());
