#!/bin/bash

STATIC="../src-tauri/static"
MODULES="$STATIC/modules.js"
INDEX_CONTENT="
<!DOCTYPE html>
<html lang="en">
  <head>
    <script type="module" src="./modules.js"></script>
    <script type="module" src="./server.js"></script>
    <script type="module" src="./installer.js"></script>"

for m in ./modules/*; do
  if [[ -f "$m" ]]; then
    cp -f "$m" "$STATIC"
  else
    pushd .
    cd "$m"
    if [[ -f "$m/package.json" ]]; then
      bun i
      bun run build.ts
    fi
    popd
  fi
done

echo "" >>"./modules.ts"
for m in ./built_modules/*; do
  echo "import '$m';" >>"./modules.ts"
done

tmpFile=$(mktemp)
echo "$INDEX_CONTENT" >>$tmpFile

pushd .
cd modules
for css in ./*.css; do
  echo "<link href='$css' rel='stylesheet' type='text/css'/>" >>$tmpFile
done
popd

echo "" >>tmpFile
echo "</head><body></body></html>" >>$tmpFile

bun run build.ts

cp $tmpFile "$STATIC/index.html"
