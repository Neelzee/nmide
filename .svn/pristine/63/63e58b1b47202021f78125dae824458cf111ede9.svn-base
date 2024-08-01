default:
  just --list

alias dev := run
alias bp := build-plugins

nmcdir := "nmide-core/src-tauri/"
nmlibc := "nmide-lib/"
thesis := "nmide-thesis"

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
  cd {{nmlibc}}/build && cmake --build .
  cp {{nmlibc}}/*.h nmide-wrapper/nmide-rust-ffi/
  cp {{nmlibc}}build/*.a  nmide-wrapper/nmide-rust-ffi/

pdf:
  pdflatex --output-directory={{thesis}} {{thesis}}/main.tex

test:
  cd {{nmcdir}} && cargo test

docker:
  docker build . -t neelzee/tauri_img:latest
  docker push neelzee/tauri_img:latest

svn:
  just clean
  svn add . --force
  svn commit -m "Push changes to SVN Repo" --username ${SVN_USERNAME} --password ${SVN_PASSWORD} --non-interactive
