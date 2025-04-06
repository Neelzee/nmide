const esbuild = require('esbuild');

esbuild.build({
  entryPoints: ['./main.ts'],
  bundle: true,
  outfile: "../../core/plugins/ide-framework.js",
  minify: true,
}).catch(() => process.exit(1));

