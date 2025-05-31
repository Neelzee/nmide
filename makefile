appdir := ~/.local/share/no.nilsmf.uib
manifest-path := tools/cm-installer/Cargo.toml
out := core/src-tauri/target
conf := Modules.toml
modules := core/modules
cargo := core/src-tauri/Cargo.toml
dist := core/build
index := core/index.html
tool-out := tools/module-tester/rsm-grapher/target
tool-cargo := tools/module-tester/rsm-grapher/Cargo.toml
tool-dist := tools/module-tester/build
DEV_MODE := $(if $(DEVELOPMENT),true,false)

.PHONY: build-modules modules install-deps clean init ide help

help:
	@echo "Available targets:"
	@echo "  help          - Show this help message"
	@echo "  install-deps  - Install JavaScript libraries"
	@echo "  init          - Initialize project structure"
	@echo "  clean         - Clean build artifacts"
	@echo "  build-modules - Build all modules"
	@echo "  modules       - Add Modules from configuration to IDE"
	@echo "  ide           - Full build for IDE"

build-modules:
	@printf "Building modules...\n"
	@$(MAKE) -C modules module-build
	@printf "✓ Modules built successfully\n"

modules: clean init
	@printf "Processing modules configuration...\n"
	@cargo run \
		--manifest-path=$(manifest-path) -- \
		--out=$(out) \
		--conf=$(conf) \
		--modules=$(modules) \
		--cargo=$(cargo) \
		--dist=$(dist)/external \
		--index=$(index) \
		--module-dist=$(appdir)/modules >/dev/null 2>&1
	@printf "✓ Module configuration processed\n"
	@printf "Building core TypeScript...\n"
	@cd core && bun run build.ts >/dev/null 2>&1
	@printf "✓ Core build completed\n"

install-deps:
	@printf "Linking JavaScript libraries...\n"
	@( \
		cd ./libs/javascript && \
		find . -maxdepth 1 -type d ! -path . | while read p; do \
			if [ -f "$$p/package.json" ]; then \
				printf "  Linking $$p... "; \
				if (cd "$$p" && bun link >/dev/null 2>&1); then \
					printf "✓\n"; \
				else \
					printf "$(RED)✗\n"; \
				fi; \
			fi; \
		done && \
		find . -maxdepth 1 -type d ! -path . | while read p; do \
			if [ -f "$$p/package.json" ]; then \
				printf "  Installing $$p... "; \
				if (cd "$$p" && bun i >/dev/null 2>&1); then \
					printf "✓\n"; \
				else \
					printf "$(RED)✗\n"; \
				fi; \
			fi; \
		done \
	)
	@printf "Installing core dependencies...\n"
	@cd core && bun i >/dev/null 2>&1
	@printf "✓ All dependencies installed\n"

clean:
	@printf "Removing build artifacts...\n"
	@rm -rf $(out)/debug/build/core-* 2>/dev/null || true
	@rm -rf $(dist)/external 2>/dev/null || true
	@if [ "$(DEV_MODE)" = "false" ] && [ -d "$(appdir)" ]; then \
		printf "WARNING: About to delete application directory: $(appdir)\n"; \
		printf "Continue? [y/N] "; \
		read -r confirm; \
		if [ "$$confirm" = "y" ] || [ "$$confirm" = "Y" ]; then \
			printf "	Removing application directory...\n"; \
			rm -rf $(appdir); \
			printf "	✓ Application directory removed\n"; \
		else \
			printf "Skipping application directory removal\n"; \
		fi; \
	else \
		printf "Removing application directory...\n"; \
		rm -rf $(appdir) 2>/dev/null || true; \
		printf "	✓ Application directory removed\n"; \
	fi
	@awk '{ print } /^# =+ #/ { exit }' $(cargo) > tmp && mv tmp $(cargo)
	@awk '/<!--MODULES-->/ { print; in_block = !in_block; next; } !in_block' $(index) > tmp && mv tmp $(index)
	@printf "✓ Build artifacts removed\n"

init:
	@printf "Initializing project structure...\n"
	@mkdir -p $(dist)/external
	@touch $(dist)/external/modules.js
	@mkdir -p $(out)
	@echo "pub fn register_modules(modules: &mut HashMap<String, Box<dyn Module>>) {}" > $(out)/module_reg.rs
	@mkdir -p $(appdir)/modules
	@printf "✓ Project structure initialized\n"

ide: init install-deps build-modules modules
	@printf "Building application for IDE/production...\n"
	@cd core && bun run tauri build >/dev/null 2>&1
	@printf "✅ IDE build completed successfully!\n"
	@printf "	Binaries can be found at "
	@(cd $(out)/release/bundle && pwd)
	@printf "	in the following folders: "
	@ls $(out)/release/bundle
