// TODO: Add docs
export const jsmInstaller = async (src: string): Promise<string | undefined> => {
  if (!src.endsWith(".js") && !src.endsWith(".mjs")) return src;
  const name = src.split("%2F").pop()?.split(".")[0];
  const script = document.createElement("script");
  script.src = src;
  script.id = name === undefined ? src.toString() : name
  script.type = src.endsWith(".mjs") ? "module" : "";
  document.head.append(script);
  return;
}
