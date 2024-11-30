import { parseHtml, renderHtml } from "./lib/renderHtml";
import { main } from "../index.js";

window.cleanup = [];
window.pluginAssets = [];
window.renderHtml = renderHtml;
window.parseHtml = parseHtml;
window.root = document.body;

main();