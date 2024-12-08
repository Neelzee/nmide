import { HtmlBuilder, TMap } from "@nmide/js-utils";

export const init = (): TMap => {

  const framework = new HtmlBuilder()
    .kind("main")
    .kids(
      new HtmlBuilder()
        .kind("div")
        .attrs({ id: "tool-bar" }),
      new HtmlBuilder()
        .kind("aside")
        .attrs({ id: "explorer" }),
      new HtmlBuilder()
        .kind("div")
        .attrs({ id: "root" }),
      new HtmlBuilder()
        .kind("div")
        .attrs({ id: "status-bar" }),
    )
    .build();

  window.renderHtml(framework);

  const root = document.getElementById("root");

  if (root !== null) {
    window.root = root;
  }

  return [];
};
