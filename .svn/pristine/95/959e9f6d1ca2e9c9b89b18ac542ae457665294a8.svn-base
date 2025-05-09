const esbuild = require('esbuild');

esbuild.build({
  entryPoints: ['./main.ts'],
  bundle: false,
  outfile: "./build/index.js",
  minify: false,
}).catch(() => process.exit(1));

