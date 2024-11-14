const esbuild = require('esbuild');

esbuild.build({
  entryPoints: ['./main.ts'],
  bundle: true,
  outfile: './dist/bundle.js',
  minify: true,
}).catch(() => process.exit(1));

