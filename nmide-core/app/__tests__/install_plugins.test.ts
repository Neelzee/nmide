import { afterEach, expect, test } from 'vitest';
import { clearMocks, mockIPC } from '@tauri-apps/api/mocks';
import { InstallPluginsFunction, LoadPluginsFunction } from "../lib/InstallPlugins";
import { NmluginVerified } from '../lib/Nmlugin';
import TrivialPlugin from './test_plugins/trivial_plugin';
import CounterPlugin from './test_plugins/counter_plugin';

afterEach(() => {
  clearMocks();
  // @ts-ignore: window.plugins should be undefined before plugin installation
  window.plugins = undefined;
});

const mockInstallation = (plugins: [string, NmluginVerified][]) => mockIPC((_, __?) => {
  window.plugins = new Map();
  plugins.forEach(([k, v]) => window.plugins.set(k, v));
  return new Promise<any>(r => r(null));
});

test("No Plugins to Install", async () => {
  mockInstallation([]);
  expect(window.plugins).toBeUndefined();
  await InstallPluginsFunction();
  expect(window.plugins).not.toBeUndefined();
});

test("No Plugins to Load", async () => {
  window.plugins = new Map();
  const plugins = await LoadPluginsFunction();
  expect(plugins.length).toBe(0);
});

test("No Plugins to Install or Load", async () => {
  mockInstallation([]);
  await InstallPluginsFunction();
  const plugins = await LoadPluginsFunction();
  expect(plugins.length).toBe(0);
});

test("Install and Load TrivialPlugin", async () => {
  mockInstallation([["Trivial Plugin", TrivialPlugin]]);
  await InstallPluginsFunction();
  const plugins = await LoadPluginsFunction();
  expect(plugins.length).toBe(1);
});

test("Install and Load CounterPlugin", async () => {
  mockInstallation([["CounterPlugin", CounterPlugin]]);
  await InstallPluginsFunction();
  const plugins = await LoadPluginsFunction();
  expect(plugins.length).toBe(1);
});


test("Install and Load CounterPlugin and TrivialPlugin", async () => {
  mockInstallation([
    ["CounterPlugin", CounterPlugin],
    ["Trivial Plugin", TrivialPlugin],
  ]);
  await InstallPluginsFunction();
  const plugins = await LoadPluginsFunction();
  expect(plugins.length).toBe(2);
});

test(
  "Installing and Loading the same plugins n-times, resolves too one of each.",
  async () => {
    mockInstallation([
      ["CounterPlugin", CounterPlugin],
      ["Trivial Plugin", TrivialPlugin],
      ["CounterPlugin", CounterPlugin],
      ["Trivial Plugin", TrivialPlugin],
      ["CounterPlugin", CounterPlugin],
      ["Trivial Plugin", TrivialPlugin],
      ["CounterPlugin", CounterPlugin],
      ["Trivial Plugin", TrivialPlugin],
    ]);
    await InstallPluginsFunction();
    const plugins = await LoadPluginsFunction();
    expect(plugins.length).toBe(2);
  }
);
