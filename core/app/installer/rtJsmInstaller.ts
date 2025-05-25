import { NMIDE_RT_MODULE_INSTALLED_EVENT } from "@nmide/js-core-std-lib";

// TODO: Add docs
export const rtJsmInstaller = async (src: string): Promise<string | undefined> => {
  if (!src.endsWith(".js") && !src.endsWith(".mjs")) return;
  const name = src.split("%2F").pop()?.split(".")[0];
  const script = document.createElement("script");
  script.src = src;
  script.id = name === undefined ? src.toString() : name
  script.type = src.endsWith(".mjs") ? "module" : "";
  script.addEventListener(
    "load",
    () => {
      setTimeout(() => {
        document.dispatchEvent(new CustomEvent(NMIDE_RT_MODULE_INSTALLED_EVENT));
      }, 100);
    },
    { once: true }
  );
  document.head.append(script);
  return name;
}
