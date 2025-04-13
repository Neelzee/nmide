import { App, AppConfig } from "./App.ts";
import {
  NMIDE_INITIALIZED,
  NMIDE_MODULES_INSTALLED_EVENT
} from "./nmideConstants.ts";

export const run = (app: App, config: Partial<AppConfig>) => {
  document.addEventListener(
    NMIDE_INITIALIZED,
    () => app.installModules()
  );
  document.addEventListener(
    NMIDE_MODULES_INSTALLED_EVENT,
    () => app.run()
  );
  app.initialize(config);
}
