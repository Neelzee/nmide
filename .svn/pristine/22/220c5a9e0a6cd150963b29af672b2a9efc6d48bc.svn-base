#!/bin/bash
dir=$(pwd)
function changelog_core_std_lib() {
  new_tag=$(
    cd core/src-tauri/core-std-lib/ && git cliff --bumped-version \
      --include-path "*/src-tauri/nmide-std-lib/**" \
      --include-path "*/src-tauri/core-std-lib/**" \
      --repository "../../.." \
      --config "../../cliff.toml"
  )
  cd core/src-tauri/core-std-lib/ && echo "csl/$new_tag" && git cliff \
    --include-path "*/src-tauri/nmide-std-lib/**" \
    --include-path "*/src-tauri/core-std-lib/**" \
    --repository "../../.." \
    --config "../../cliff.toml" >CHANGELOG.md
  cd $dir
}

function changelog_core_plugin_lib() {
  new_tag=$(
    cd core/src-tauri/core-plugin-lib/ && git cliff --bumped-version \
      --include-path "*/src-tauri/nmide-plugin-manager/**" \
      --include-path "*/src-tauri/core-plugin-lib/**" \
      --repository "../../.." \
      --config "../../cliff.toml"
  )
  cd core/src-tauri/core-plugin-lib/ && echo "cpl/$new_tag" && git cliff \
    --include-path "*/src-tauri/nmide-plugin-manager/**" \
    --include-path "*/src-tauri/core-plugin-lib/**" \
    --repository "../../.." \
    --config "../../cliff.toml" >CHANGELOG.md
  cd $dir
}

function changelog_core_macros() {
  new_tag=$(
    cd core/src-tauri/core-macros/ && git cliff --bumped-version \
      --include-path "*/src-tauri/nmide-macros/**" \
      --include-path "*/src-tauri/core-macros/**" \
      --repository "../../.." \
      --config "../../cliff.toml"
  )
  cd core/src-tauri/core-macros/ && echo "cms/$new_tag" && git cliff \
    --include-path "*/src-tauri/nmide-macros/**" \
    --include-path "*/src-tauri/core-macros/**" \
    --repository "../../.." \
    --config "../../cliff.toml" >CHANGELOG.md
  cd $dir
}

function changelog_core() {
  new_tag=$(
    cd core && git cliff --bumped-version \
      --include-path "core/**" \
      --include-path "nmide-core/**" \
      --exclude-path "core/src-tauri/nmide-std-lib/**" \
      --exclude-path "core/src-tauri/core-std-lib/**" \
      --exclude-path "core/src-tauri/nmide-plugin-manager/**" \
      --exclude-path "core/src-tauri/core-plugin-lib/**" \
      --exclude-path "core/src-tauri/nmide-macros/**" \
      --exclude-path "core/src-tauri/core-macros/**" \
      --repository "../" \
      --config "cliff.toml"
  )
  cd core && echo "nc/$new_tag" && git cliff \
    --include-path "core/**" \
    --include-path "nmide-core/**" \
    --exclude-path "core/src-tauri/nmide-std-lib/**" \
    --exclude-path "core/src-tauri/core-std-lib/**" \
    --exclude-path "core/src-tauri/nmide-plugin-manager/**" \
    --exclude-path "core/src-tauri/core-plugin-lib/**" \
    --exclude-path "core/src-tauri/nmide-macros/**" \
    --exclude-path "core/src-tauri/core-macros/**" \
    --repository "../" \
    --config "cliff.toml" >CHANGELOG.md
  cd $dir
}

function changelog_c_lib() {
  new_tag=$(
    cd libs/c-lib && git cliff --bumped-version \
      --include-path "libs/c-lib/**" \
      --include-path "nmide-lib/**" \
      --repository "../.." \
      --config "cliff.toml"
  )
  cd libs/c-lib && echo "cl/$new_tag" && git cliff \
    --include-path "libs/c-lib/**" \
    --include-path "nmide-lib/**" \
    --repository "../../" \
    --config "cliff.toml" >CHANGELOG.md
  cd $dir
}

function changelog_js_lib() {
  new_tag=$(
    cd libs/js-utils && git cliff --bumped-version \
      --include-path "libs/js-utils/**" \
      --repository "../.." \
      --config "cliff.toml"
  )
  cd libs/js-utils && echo "jsl/$new_tag" && git cliff \
    --include-path "libs/js-utils/**" \
    --repository "../../" \
    --config "cliff.toml" >CHANGELOG.md
  cd $dir
}

function changelog() {
  git cliff \
    --config "cliff.toml" >CHANGELOG.md
}

function main() {
  changelog
  changelog_js_lib
  changelog_c_lib
  changelog_core_plugin_lib
  changelog_core_macros
  changelog_core_std_lib
  changelog_core
}

main
