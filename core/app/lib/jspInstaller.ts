export const jspInstaller = async (src: string): Promise<string | undefined> => {
  if (!src.endsWith(".js")) return src;
  let name = src.split("%2F").pop()?.split(".")[0];
  name = name === undefined ? src : name
  window.pluginAssets.push([name, src]);
  const script = document.createElement("script");
  script.src = src;
  document.head.append(script);
  return;
}
