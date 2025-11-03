#!/bin/bash

cargo run \
  --manifest-path ./src-tauri/Cargo.toml \
  --features server server
