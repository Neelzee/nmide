manifest-path :=  ./src-tauri/Cargo.toml
appdir := /home/nmf/.local/share/no.nilsmf.uib
out := ./src-tauri/target
conf := Modules.toml
modules := ./modules
cargo := ./src-tauri/Cargo.toml
dist := ./dist
index := ./index.html
tool-out := tools/module-tester/rsm-grapher/target
tool-cargo := tools/module-tester/rsm-grapher/Cargo.toml
tool-dist := tools/module-tester/build


modules : export DIST_DIR=$(dist)
modules : export RUST_LOG=debug
modules : export MODULE_CONFIG=$(conf)
modules : export INDEX_PATH=$(index)
modules : export CARGO_PATH=$(manifest-path)
modules : clean
	cargo run \
		--manifest-path=$(manifest-path) -- \
		install && \
	bun run build.ts

clean : export DIST_DIR=$(dist)
clean : export RUST_LOG="debug"
clean : export MODULE_CONFIG=$(conf)
clean : export INDEX_PATH=$(index)
clean : export CARGO_PATH=$(manifest-path)
clean :
	rm -rf $(out)/debug/build/core-* && \
	rm -rf $(dist)/external && \
	rm -rf $(appdir)/modules && \
	mkdir -p $(dist)/external && \
	touch $(dist)/external/modules.js && \
	mkdir -p $(out) && \
	touch $(out)/module_reg.rs && \
	echo "pub fn register_modules(modules: &mut HashMap<String, Box<dyn Module>>) {}" \
		> $(out)/module_reg.rs && \
	mkdir -p $(appdir)/modules && \
	awk '{ print } /^# =+ #/ { exit }' $(cargo) > tmp && mv tmp $(cargo) && \
	cargo run \
		--features "module-installer" \
		--manifest-path=$(manifest-path) \
		-- \
		clean
