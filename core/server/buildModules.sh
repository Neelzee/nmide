#!/bin/bash

STATIC="../src-tauri/static"
MODULES="$STATIC/modules.js"
BUILT_MODULES="./built_modules"
INDEX_CONTENT="<!DOCTYPE html>
<html lang="en">
  <head>
    <script type="module" src="./modules.js"></script>
    <script type="module" src="./server.js"></script>"

if [[ ! -d $STATIC ]]; then
  mkdir -p $STATIC
fi

if [[ ! -d $BUILT_MODULES ]]; then
  mkdir -p $BUILT_MODULES
else
  rm -rf $BUILT_MODULES
  mkdir -p $BUILT_MODULES
fi

for m in ./modules/*; do
  if [[ -f "$m" ]]; then
    cp -f "$m" "$STATIC"
  else
    pushd .
    cd "$m"
    if [[ -f "$m/package.json" ]]; then
      bun i
    fi
    popd
  fi
done

tmpFile=$(mktemp)
echo "$INDEX_CONTENT" >>$tmpFile

pushd .
cd modules
for css in ./*.css; do
  echo "<link href='$css' rel='stylesheet' type='text/css'/>" >>$tmpFile
done
popd

echo "" >>$tmpFile
echo "</head><body></body></html>" >>$tmpFile

bun run build.ts

cp $tmpFile "$STATIC/index.html"
