import { PartialTMapFieldOrd, tBool, tLookupOr, TMap } from "@nmide/js-utils";
import { renderTable } from "./view";
import { sort } from "fp-ts/lib/Array";
import { toArray } from "fp-ts/lib/Map";
import { Ord } from "fp-ts/lib/string";

export const reRender = (model: TMap) => {
  // renders Model
  const html = renderTable(sort(PartialTMapFieldOrd)(model));
  const table = document.getElementById("reggub-state-table");
  if (table === null) {
    return;
  };
  table.textContent = "";
  const elem = window.parseHtml(html);
  table.appendChild(elem);
};
