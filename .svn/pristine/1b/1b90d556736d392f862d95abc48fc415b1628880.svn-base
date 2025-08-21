// TODO: Add docs
export const cssInstaller = async (src: string): Promise<string | undefined> => {
  if (!src.endsWith(".css")) return src;
  const style = document.createElement("link");
  style.href = src;
  style.rel = "stylesheet";
  document.head.append(style);
  return;
}
