"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var js_utils_1 = require("@nmide/js-utils");
// @ts-ignore
window.plugins.set("DependencyViewer", {
  init: function() {
    return [];
  },
  view: function(model) {
    return new js_utils_1.HtmlBuilder()
      .kids([
        new js_utils_1.HtmlBuilder()
          .kind("Div")
          .attrs([{ Id: "container" }]),
      ])
      .build();
  },
  update: function(msg, model) {
    return [];
  },
});
