// TODO: Add docs
export const cssInstaller = async (src: string): Promise<string | undefined> => {
  if (!src.endsWith(".css")) return src;
  let name = src.split("%2F").pop()?.split(".")[0];
  name = name === undefined ? src : name
  window.pluginAssets.push([name, src]);
  const style = document.createElement("link");
  style.href = src;
  style.rel = "stylesheet";
  document.head.append(style);
  return;
}
