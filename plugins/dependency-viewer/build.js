const esbuild = require('esbuild');

esbuild.build({
  entryPoints: ['./main.ts'],
  bundle: true,
  outfile: './dist/DependencyViewer.js',
  minify: true,
}).catch(() => process.exit(1));

