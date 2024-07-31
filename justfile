default:
  just --list

alias dev := run
alias bp := build-plugins

nmcdir := "nmide-core/src-tauri/"
nmlibc := nmcdir / "nmide-libc/"

run:
  npm run tauri dev

clean:
  cd {{nmcdir}}nmide-rust-ffi && cargo clean && rm *.h ./html/*.h
  cd nmide-core/src-tauri && cargo clean
  cd {{nmlibc}} && make clean
  rm -rf dist
  rm {{nmcdir}}nmide-rust-ffi/bindings/*.ts

build:
  -cd {{nmcdir}}nmide-rust-ffi && cargo test && cargo build --release
  cp {{nmcdir}}nmide-rust-ffi/bindings/*.ts nmide-core/src/bindings/
  cd {{nmlibc}} && make
  cd nmide-core/ && npm run tauri build

build-plugins:
  cd {{nmcdir}}nmide-framework && cargo build --release
  cp {{nmcdir}}target/release/libnmide_framework.so {{nmcdir}}plugin-libs/

make:
  cd {{nmlibc}} && make clean && make
  cp {{nmlibc}}/html/*.h {{nmcdir}}nmide-rust-ffi/html/
  cp {{nmlibc}}/*.h {{nmcdir}}nmide-rust-ffi/
  cp {{nmlibc}}/*.so  {{nmcdir}}nmide-rust-ffi/

test:
  cd {{nmcdir}} && cargo test
