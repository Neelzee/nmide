import Nmlugin from "./Nmlugin";

declare global {
  interface Window { plugins: Nmlugin[]; }
}
