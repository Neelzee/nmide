#!/bin/bash

STATIC="./static"
MODULES="$STATIC/modules.js"

for m in ./modules/*; do
  pushd .
  cd "$m"
  bun i
  bun run build.ts
  popd
done

echo "" >>"./modules.ts"
for m in ./built_modules/*; do
  echo "import '$m';" >>"./modules.ts"
done

bun run build.ts
