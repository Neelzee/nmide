default:
  just --list

alias dev := run
alias bp := build-plugins

nmcdir := "nmide-core/src-tauri/"
nmlibc := "nmide-lib/"

run:
  npm run tauri dev

clean:
  cd nmide-wrapper/nmide-rust-ffi && cargo clean && rm *.h ./html/*.h
  cd nmide-core/src-tauri && cargo clean
  cd {{nmlibc}} && make clean
  rm -rf dist
  rm nmide-wrapper/nmide-rust-ffi/bindings/*.ts

build:
  -cd nmide-wrapper/nmide-rust-ffi && cargo test && cargo build --release
  cp nmide-wrapper/nmide-rust-ffi/bindings/*.ts nmide-core/src/bindings/
  cd {{nmlibc}} && make
  cd nmide-core/ && npm run tauri build

build-plugins:
  cd nmide-plugin/nmide-framework && cargo build --release
  cp nmide-plugin/nmide-framework/target/release/libnmide_framework.so {{nmcdir}}plugin-libs/

build-release:
  cd nmide-core && npm i && npm run tauri build

make:
  cd {{nmlibc}} && make clean && make
  cp {{nmlibc}}/html/*.h nmide-wrapper/nmide-rust-ffi/html/
  cp {{nmlibc}}/*.h nmide-wrapper/nmide-rust-ffi/
  cp {{nmlibc}}/*.so  nmide-wrapper/nmide-rust-ffi/

test:
  cd {{nmcdir}} && cargo test
