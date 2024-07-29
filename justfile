default:
  just --list

alias dev := run
alias bp := build-plugins

run:
  npm run tauri dev

clean:
  cd src-tauri/nmide-rust-ffi && cargo clean && rm *.h ./html/*.h
  cd src-tauri && cargo clean
  cd src-tauri/nmide-libc && make clean
  rm -rf dist
  rm src-tauri/nmide-rust-ffi/bindings/*.ts

build:
  -cd src-tauri/nmide-rust-ffi && cargo test && cargo build --release
  cp src-tauri/nmide-rust-ffi/bindings/*.ts src/bindings/
  cd src-tauri/nmide-libc && make
  npm run tauri build

build-plugins:
  cd src-tauri/nmide-framework && cargo build --release
  cp src-tauri/target/release/libnmide_framework.so src-tauri/plugin-libs/

make:
  cd src-tauri/nmide-libc && make clean && make
  cp src-tauri/nmide-libc/html/*.h src-tauri/nmide-rust-ffi/html/
  cp src-tauri/nmide-libc/*.h src-tauri/nmide-rust-ffi

test:
  cd src-tauri && cargo test
