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
    // TODO: Remove when @nmide/js-utils is stable.
    // Because @nmide/js-utils is _installed_ by referencing
    // it as a path, it needs to be installed for the core to work.
    fs: {
      allow: [".."],
    }
  },
  // Env variables starting with the item of `envPrefix` will be exposed in tauri's source code through `import.meta.env`.
  envPrefix: ['VITE_', 'TAURI_ENV_*'],
  build: {
    // Tauri uses Chromium on Windows and WebKit on macOS and Linux
    target:
      process.env.TAURI_ENV_PLATFORM == 'windows'
        ? 'chrome105'
        : 'safari13',
    // don't minify for debug builds
    minify: !process.env.TAURI_ENV_DEBUG ? 'esbuild' : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
    rollupOptions: {
      // HACK: This should not be needed, I think. But makes it so that I can build
      external: [
        "fp-ts/",
        "fp-ts/function",
        "fp-ts/lib/function",
        "fp-ts/lib/Ord",
        "fp-ts/Array",
        "fp-ts/NonEmptyArray",
        "fp-ts/Tuple",
        "fp-ts/string",
        "fp-ts/boolean",
        "fp-ts/Option",
        "fp-ts/number",
        "fp-ts/Either",
        "fp-ts/Eq",
        "io-ts",
      ]
    },
  },
  plugins: [
    nodePolyfills({ include: [] }),
  ],
});
