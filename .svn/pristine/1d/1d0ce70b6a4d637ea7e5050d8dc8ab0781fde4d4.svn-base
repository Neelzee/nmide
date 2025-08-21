import { defineConfig } from 'vite';
import { nodePolyfills } from "vite-plugin-node-polyfills";

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  // prevent vite from obscuring rust errors
  clearScreen: false,
  server: {
    // Tauri expects a fixed port, fail if that port is not available
    strictPort: true,
    // if the host Tauri is expecting is set, use it
    host: host || false,
    port: 5173,
    // TODO: Remove when @nmide/js-utils is stable
    fs: {
      allow: [".."],
    }
  },
  envPrefix: ["VITE_"],
  build: {
    target: "chrome105",
    minify: true,
  },
  plugins: [
    nodePolyfills({ include: [] }),
  ],
});
