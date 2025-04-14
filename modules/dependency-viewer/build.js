const esbuild = require('esbuild');

esbuild.build({
  entryPoints: ['./main.ts'],
  bundle: true,
  outfile: "../../core/modules/dependency_viewer.js",
  minify: true,
}).catch(() => process.exit(1));

