import { AppConfig } from "@nmide/js-utils";
import {
  NMIDE_INITIALIZED,
  NMIDE_MODULES_INSTALLED_EVENT
} from "./nmideConstants.ts";
import { App } from "@nmide/js-core-std-lib";

export const run = (app: App, config: Partial<AppConfig>) => {
  document.addEventListener(
    NMIDE_INITIALIZED,
    app.installModules,
    { once: true }
  );
  document.addEventListener(
    NMIDE_MODULES_INSTALLED_EVENT,
    app.run,
    { once: true }
  );
  app.initialize(config);
}
