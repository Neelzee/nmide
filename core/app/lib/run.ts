import { type AppConfig } from "@nmide/js-utils";
import {
  type App,
  NMIDE_INITIALIZED,
} from "@nmide/js-core-std-lib";

export const run = (app: App, config: Partial<AppConfig>) => {
  document.addEventListener(
    NMIDE_INITIALIZED,
    app.run,
    { once: true }
  );
  app.initialize(config);
}
