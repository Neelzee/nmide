const esbuild = require('esbuild');

esbuild.build({
  entryPoints: ["./index.ts"],
  bundle: true,
  outfile: "build/index.js",
  minify: true,
  external: [],
  loader: {
    ".ttf": "file"
  }
}).catch(() => process.exit(1));

