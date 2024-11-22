import { init } from "./init";
import { update } from "./update";
import { view } from "./view";

//@ts-ignore
window.plugins.set(
  "reggub",
  {
    init,
    update,
    view,
  }
);
