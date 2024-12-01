import { parseHtml, renderHtml } from "./lib/renderHtml";

export const setup = () => {
  window.cleanup = [];
  window.pluginAssets = [];
  window.renderHtml = renderHtml;
  window.parseHtml = parseHtml;
  window.root = document.body;
};

