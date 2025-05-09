import * as esbuild from 'esbuild';

esbuild.build({
  entryPoints: ["./index.ts"],
  bundle: true,
  outfile: "build/index.js",
  minify: true,
  external: ["fp-ts/*", "io-ts"]
}).catch(console.error);

