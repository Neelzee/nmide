import "@nmide/js-utils";
import { EpicInstallPlugin } from "./SuperEpicPluginInstaller";

export const InstallPlugins = async (): Promise<void> => {
  try {
    await EpicInstallPlugin();
  } catch (err) {
    console.error("Install Error: ", err);
  }
};

