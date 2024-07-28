default:
  just --list

run:
  npm run tauri dev

clean:
  cd src-tauri/nmide-rust-ffi && cargo clean
  cd src-tauri && cargo clean
  cd src-tauri/nmide-libc && make clean

build:
  -cd src-tauri/nmide-rust-ffi && cargo build --release && cargo run --features headers --bin generate-headers
  cd src-tauri/nmide-libc && make
  npm run tauri build
