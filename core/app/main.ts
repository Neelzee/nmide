import { App } from "./App";
import { renderHtml } from "./lib/renderHtml";

window.renderHtml = renderHtml;
window.addEventListener("DOMContentLoaded", () => App());
