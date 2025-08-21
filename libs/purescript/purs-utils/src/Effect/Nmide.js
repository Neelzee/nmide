export const getPlugins = Array.from(window.plugins.entries());

export const addPlugin = (pluginName) => (plugin) => window.plugins.set(pluginName, plugin);