#!/bin/bash

main() {
  find . -name "package.json" | grep -v "node_modules" | while read module; do
    linkModule "$module"
  done
}

linkModule() {
  local module
  module="$1"
  local path
  path=${module%"package.json"}
  jq -r ".dependencies[]" "$module" | while read dep; do
    case "$dep" in
      link:*)
        pushd .
        cd "$path"
        bun link "$dep"
        popd
        ;;
    esac
  done
}

main
