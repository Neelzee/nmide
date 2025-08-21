const esbuild = require('esbuild');

esbuild.build({
  entryPoints: ['./index.ts'],
  bundle: true,
  outfile: "../../core/plugins/menu.js",
  minify: true,
}).catch(() => process.exit(1));

