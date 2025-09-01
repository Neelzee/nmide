#!/bin/bash

pushd .
cd server
bash ./buildModules.sh
popd
pushd .
cd src-tauri
cargo build --release --features server
docker build . -t nmide-server
docker image tag nmide-server neelzee/nmide-server:latest
docker image push neelzee/nmide-server:latest
