import "@nmide/js-utils";
import { init } from "./init";
import { update } from "./update";
import { view } from "./view";

export const pluginName = "ide-framework";

window.plugins.set(
  pluginName,
  {
    init,
    update,
    view,
  }
)
