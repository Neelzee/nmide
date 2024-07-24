default:
  just --list

run:
  npm run tauri dev

build:
  -cd src-tauri/nmide-ffi && cargo build --release && cargo run --features headers --bin generate-headers
  npm run tauri build
