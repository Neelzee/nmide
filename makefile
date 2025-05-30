appdir := ~/.local/share/no.nilsmf.uib
manifest-path :=  tools/cm-installer/Cargo.toml
out := core/src-tauri/target
conf := Modules.toml
modules := core/modules
cargo := core/src-tauri/Cargo.toml
dist := core/build
index := core/index.html
tool-out := tools/module-tester/rsm-grapher/target
tool-cargo := tools/module-tester/rsm-grapher/Cargo.toml
tool-dist := tools/module-tester/build

modules : clean
	cargo run \
		--manifest-path=$(manifest-path) -- \
		--out=$(out) \
		--conf=$(conf) \
		--modules=$(modules) \
		--cargo=$(cargo) \
		--dist=$(dist)/external \
		--index=$(index) \
		--module-dist=$(appdir)/modules && \
	cd core && bun run build.ts && cd -

install-module-deps :
	@( cd modules && ./js-modules.sh )

install-deps:
	@( \
		cd ./libs/javascript && \
		find . -maxdepth 1 -type d ! -path . | while read p; do \
			if [ -f "$$p/package.json" ]; then \
				echo "Linking $$p"; \
				(cd "$$p" && bun link); \
			fi; \
		done && \
		find . -maxdepth 1 -type d ! -path . | while read p; do \
			if [ -f "$$p/package.json" ]; then \
				echo "Installing in $$p"; \
				(cd "$$p" && bun i); \
			fi; \
		done \
	) && \
	cd core && echo "Installing Core JS dependencies" && bun i && cd -

clean :
	rm -rf $(out)/debug/build/core-* && \
	rm -rf $(dist)/external && \
	rm -rf $(appdir) && \
	mkdir -p $(dist)/external && \
	touch $(dist)/external/modules.js && \
	mkdir -p $(out) && \
	touch $(out)/module_reg.rs && \
	echo "pub fn register_modules(modules: &mut HashMap<String, Box<dyn Module>>) {}" \
		> $(out)/module_reg.rs && \
	mkdir -p $(appdir)/modules && \
	awk '{ print } /^# =+ #/ { exit }' $(cargo) > tmp && mv tmp $(cargo) && \
	awk '/<!--MODULES-->/ { print; in_block = !in_block; next; } !in_block' $(index) > tmp && mv tmp $(index)

ide : install-deps install-module-deps modules
	@( cd core && bun run tauri build )
