const esbuild = require('esbuild');

esbuild.build({
  entryPoints: ["./index.ts"],
  bundle: true,
  outfile: "../../core/plugins/ide-errors.js",
  minify: true,
}).catch(() => process.exit(1));

