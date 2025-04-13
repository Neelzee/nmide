// TODO: Add docs
export const jsmInstaller = async (src: string): Promise<string | undefined> => {
  if (!src.endsWith(".js")) return src;
  const script = document.createElement("script");
  script.src = src;
  document.head.append(script);
  return;
}
