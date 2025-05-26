appdir := /home/nmf/.local/share/no.nilsmf.uib
manifest-path :=  tools/cm-installer/Cargo.toml
out := core/src-tauri/target
conf := Modules.toml
modules := core/modules
cargo := core/src-tauri/Cargo.toml
dist := core/dist
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
	cd core && bun run build.ts && cd .. && \
	cargo run \
		--manifest-path=$(manifest-path) -- \
		--out=$(tool-out) \
		--conf=$(conf) \
		--cargo=$(tool-cargo) \
		--dist=$(tool-dist)

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
	cd core && echo "Installing Core JS dependencies" && bun i

clean :
	rm -rf $(tool-out)/debug/build/core-* && \
	mkdir -p $(tool-out) && \
	rm -rf $(tool-dist) && \
	mkdir -p $(tool-dist) && \
	touch $(tool-dist)/result.json && \
	touch $(tool-dist)/index.html && \
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
	cargo run \
		--manifest-path=$(manifest-path) -- \
		--out=$(out) \
		--clean \
		--conf=$(conf) \
		--modules=$(modules) \
		--cargo=$(cargo) \
		--dist=$(dist)/external \
		--index=$(index)
	cargo run \
		--manifest-path=$(manifest-path) -- \
		--clean \
		--out=$(tool-out) \
		--conf=$(conf) \
		--cargo=$(tool-cargo) \
		--dist=$(tool-dist) \
		--index=$(tool-dist)/index.html

dry :
	cargo run \
		--manifest-path=$(manifest-path) -- \
		--out=$(out) \
		--conf=$(conf) \
		--modules=$(modules) \
		--cargo=$(cargo) \
		--dist=$(dist)/external \
		--index=$(index) \
		--module-dist=$(appdir)/modules \
		--dry-run

build : modules
	@( cd core && bun run tauri build )

run : modules
	@( cd core && bun run tauri dev )
