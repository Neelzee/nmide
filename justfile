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
  -cd nmide-wrapper/nmide-rust-ffi &&  cargo build --release
  -cp nmide-wrapper/nmide-rust-ffi/bindings/*.ts nmide-core/src/bindings/

build-plugins:
  cd nmide-plugin/nmide-framework && cargo build --release
  cp nmide-plugin/nmide-framework/target/release/libnmide_framework.so {{nmcdir}}plugin-libs/

build-release:
  just init
  just make
  cd nmide-core && npm i && npm run tauri build

make:
  cd {{nmlibc}}build && make
  cp {{nmlibc}}*.h nmide-wrapper/nmide-rust-ffi/
  cp {{nmlibc}}html/*.h nmide-wrapper/nmide-rust-ffi/html
  cp {{nmlibc}}build/*.a  nmide-wrapper/nmide-rust-ffi/

pdf:
  pdflatex --output-directory={{thesis}} {{thesis}}/main.tex

test-all:
  cd {{nmcdir}} && cargo test
  ./{{nmlibc}}/nmide_test

# Builds Docker Images
docker-build:
  docker build -f nmide-docker/tauri.Dockerfile . -t nmide-tauri:latest # Tauri
  docker build -f nmide-docker/full.Dockerfile . -t nmide-full:latest # Full
  docker build -f nmide-docker/thesis.Dockerfile . -t nmide-thesis:latest # Thesis
  docker build -f nmide-docker/rust.Dockerfile . -t nmide-rust:latest # Rust Testing
  docker build -f nmide-docker/node.Dockerfile . -t nmide-node:latest # Node Testing
  docker build -f nmide-docker/c.Dockerfile . -t nmide-c:latest # C Testing
  

# Tags Docker Images for release
docker-tag:
  docker tag nmide-tauri:latest {{docker_user}}/nmide-tauri:latest
  docker tag nmide-full:latest {{docker_user}}/nmide-full:latest
  docker tag nmide-thesis:latest {{docker_user}}/nmide-thesis:latest
  docker tag nmide-rust:latest {{docker_user}}/nmide-rust:latest
  docker tag nmide-node:latest {{docker_user}}/nmide-node:latest
  docker tag nmide-c:latest {{docker_user}}/nmide-c:latest

# Publishes Docker Images
docker-push:
  docker push {{docker_user}}/nmide-tauri:latest
  docker push {{docker_user}}/nmide-full:latest
  docker push {{docker_user}}/nmide-thesis:latest
  docker push {{docker_user}}/nmide-rust:latest
  docker push {{docker_user}}/nmide-node:latest
  docker push {{docker_user}}/nmide-c:latest

docker-full:
  just docker-build
  just docker-tag
  just docker-push

svn:
  just clean
  svn add . --force
  svn commit -m "Push changes to SVN Repo" --username ${SVN_USERNAME} --password ${SVN_PASSWORD} --non-interactive

init:
  -cd {{nmlibc}} && git clone https://github.com/nemequ/munit.git
  -mkdir {{nmlibc}}build
  -cd {{nmlibc}}build && export CC=gcc && cmake ..

make-clean:
  rm -rf {{nmlibc}}build
  rm -rf {{nmlibc}}munit

check:
  cd {{nmlibc}} && cppcheck --enable=all --force --quiet -imunit -ibuild .

c-test:
  cd {{nmlibc}}debug && make && ./nmide_test

make-release:
  mkdir -p {{nmlibc}}release
  @cd {{nmlibc}}release && pwd
  cd {{nmlibc}}release && cmake -DCMAKE_BUILD_TYPE=Release ..
  cd {{nmlibc}}release && make

make-test:
  cd {{nmlibc}}debug && make && ./test_cmap
  cd {{nmlibc}}debug && make && ./test_cmodel

make-check:
  cd {{nmlibc}} && cppcheck --enable=all --force --quiet -imunit -ibuild -idebug -irelease .
  cd {{nmlibc}} && valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes --verbose ./test_cmodel
  cd {{nmlibc}} && valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes --verbose ./test_cmap
