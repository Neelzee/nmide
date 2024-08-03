default:
  just --list

alias dev := run
alias bp := build-plugins

nmcdir := "nmide-core/src-tauri/"
nmlibc := "nmide-lib/"
thesis := "nmide-thesis"
docker_user := "neelzee"

run:
  npm run tauri dev

clean:
  cd nmide-wrapper/nmide-rust-ffi && cargo clean && rm -f *.h ./html/*.h
  cd nmide-core/src-tauri && cargo clean
  rm -rf nmide-core/dist
  rm -rf nmide-core/node_modules
  rm -rf nmide-core/src-tauri/target
  rm -rf nmide-plugin/*/target
  rm -rf nmide-wrapper/nmide-rust-ffi/target
  rm -f nmide-thesis/**.aux
  rm -f nmide-thesis/**.log
  rm -f nmide-thesis/**.pdf
  rm -f nmide-wrapper/nmide-rust-ffi/bindings/*.ts

build:
  just make
  cd nmide-wrapper/nmide-rust-ffi && bindgen nmidelib.h -o src/bindings.rs
  -cd nmide-wrapper/nmide-rust-ffi && cargo test && cargo build --release
  cp nmide-wrapper/nmide-rust-ffi/bindings/*.ts nmide-core/src/bindings/
  cd nmide-core/ && npm run tauri build

build-plugins:
  cd nmide-plugin/nmide-framework && cargo build --release
  cp nmide-plugin/nmide-framework/target/release/libnmide_framework.so {{nmcdir}}plugin-libs/

build-release:
  just make
  cd nmide-core && npm i && npm run tauri build

make:
  -just init
  cd {{nmlibc}} && cmake --build .
  cp {{nmlibc}}*.h nmide-wrapper/nmide-rust-ffi/
  cp {{nmlibc}}*.a  nmide-wrapper/nmide-rust-ffi/

pdf:
  pdflatex --output-directory={{thesis}} {{thesis}}/main.tex

test:
  cd {{nmcdir}} && cargo test

docker-build: # Builds Docker Images
  docker build -f nmide-docker/Dockerfile.tauri . -t nmide-tauri:latest # Tauri
  docker build -f nmide-docker/Dockerfile.full . -t nmide-full:latest # Full
  docker build -f nmide-docker/Dockerfile.thesis . -t nmide-thesis:latest # Thesis

docker-tag: # Tags Docker Images for release
  docker tag nmide-tauri:latest {{docker_user}}/nmide-tauri:latest
  docker tag nmide-full:latest {{docker_user}}/nmide-full:latest
  docker tag nmide-thesis:latest {{docker_user}}/nmide-thesis:latest

docker-push: # Publishes Docker Images
  docker push {{docker_user}}/nmide-tauri:latest
  docker push {{docker_user}}/nmide-full:latest
  docker push {{docker_user}}/nmide-thesis:latest

docker-full:
  just docker-build
  just docker-tag
  just docker-push

svn:
  just clean
  svn add . --force
  svn commit -m "Push changes to SVN Repo" --username ${SVN_USERNAME} --password ${SVN_PASSWORD} --non-interactive

init:
  -mkdir {{nmlibc}}CMakeFiles
  cd {{nmlibc}} && cmake .

make-clean:
  rm -rf {{nmlibc}}CMakeFiles {{nmlibc}}CMakeCache.txt
