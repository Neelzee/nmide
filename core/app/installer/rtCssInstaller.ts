/**
 * Runtime CSS _module_ installer.
 *
 * A CSS is not technically a module, in the sense we have used, as it does not
 * expose the `init` and `handler` methods, but its way easier to allow for CSS
 * to be _installed_, than it is to create a style attribute that maps
 * one-to-one onto the CSS API.
 *
 * _Installs_ CSS by adding a `<link />` element to the `document`, with a
 * `href` that references the `$APPDIR/modules` folder.
 *
 * @see [Tauri Permissions](https://tauri.app/security/permissions/)
 */
export const rtCssInstaller = async (src: string): Promise<string | undefined> => {
  if (!src.endsWith(".css")) return;
  const name = src.split("%2F").pop()?.split(".")[0];
  const style = document.createElement("link");
  style.href = src;
  style.rel = "stylesheet";
  document.head.append(style);
  return name;
}
