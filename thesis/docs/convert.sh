#!/bin/bash

function create_doc() {
  local path=$(dirname $1)
  local fullname=$(basename $1)
  local name="${fullname%.*}"
  mkdir -p "nmide.wiki/$path"
  touch "nmide.wiki/$path/$name.md"
  pandoc $1 --to gfm >"nmide.wiki/$path/$name.md"
}

for file in $(find "./" -type f -name "*.tex"); do
  create_doc $file
done
