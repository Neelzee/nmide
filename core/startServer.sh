#!/bin/bash

pushd .
cd src-tauri && cargo run --features server server
popd
