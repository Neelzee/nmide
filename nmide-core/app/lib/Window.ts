import { NmluginUnknown as Nmlugin } from "@nmide/js-utils";

declare global {
  interface Window { plugins: Map<string, Nmlugin>; }
}
