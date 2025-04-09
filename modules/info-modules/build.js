const esbuild = require('esbuild');

esbuild.build({
  entryPoints: ["./info_modules_helper.ts"],
  bundle: true,
  outfile: "../../core/plugins/info_modules_helper.js",
  minify: true,
}).catch(() => process.exit(1));

