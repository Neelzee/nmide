import "@nmide/js-utils";

export const InstallPlugins = async () => {
  let plugins = await window.getPluginPaths;
  const installers = window.pluginInstallers;
  for (let i = 0; i <= installers.length; i++) {
    const installer = installers[i];
    if (installer === undefined) continue;
    const promises = plugins.map(installer);
    const list = await Promise.all(promises);
    const newPlugins = list.filter(v => v !== undefined);
    plugins = newPlugins;
  }
};

