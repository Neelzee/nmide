APPDIR := $(shell echo ~/.local/share/no.nilsmf.uib)
MANIFEST-PATH := tools/cm-installer/Cargo.toml
OUT := core/src-tauri/target
CONF := Modules.toml
MODULES := core/modules
CARGO := core/src-tauri/Cargo.toml
DIST := core/build
INDEX := core/index.html
TOOL-OUT := tools/module-tester/rsm-grapher/target
TOOL-CARGO := tools/module-tester/rsm-grapher/Cargo.toml
TOOL-DIST := tools/module-tester/build
DEV_MODE := $(if $(DEVELOPMENT),true,false)
SRC_FOLDERS := \
	core \
	libs \
	modules \
	tools

.PHONY: build-modules modules install-deps clean init ide help prod

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
		--manifest-path=$(MANIFEST-PATH) -- \
		--out=$(OUT) \
		--conf=$(CONF) \
		--modules=$(MODULES) \
		--cargo=$(CARGO) \
		--dist=$(DIST)/external \
		--index=$(INDEX) \
		--module-dist=$(APPDIR)/modules >/dev/null 2>&1
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
					printf "✗\n"; \
				fi; \
			fi; \
		done && \
		find . -maxdepth 1 -type d ! -path . | while read p; do \
			if [ -f "$$p/package.json" ]; then \
				printf "  Installing $$p... "; \
				if (cd "$$p" && bun i >/dev/null 2>&1); then \
					printf "✓\n"; \
				else \
					printf "✗\n"; \
				fi; \
			fi; \
		done \
	)
	@printf "Installing core dependencies...\n"
	@cd core && bun i >/dev/null 2>&1
	@printf "✓ All dependencies installed\n"

clean:
	@printf "Removing build artifacts...\n"
	@rm -rf $(OUT)/debug/build/core-* 2>/dev/null || true
	@rm -rf $(DIST)/external 2>/dev/null || true
	@if [ "$(DEV_MODE)" = "false" ] && [ -d "$(APPDIR)" ]; then \
		printf "WARNING: About to delete application directory: $(APPDIR)\n"; \
		printf "Continue? [y/N] "; \
		read -r confirm; \
		if [ "$$confirm" = "y" ] || [ "$$confirm" = "Y" ]; then \
			printf "	Removing application directory...\n"; \
			rm -rf $(APPDIR); \
			printf "	✓ Application directory removed\n"; \
		else \
			printf "Skipping application directory removal\n"; \
		fi; \
	else \
		printf "Removing application directory...\n"; \
		rm -rf $(APPDIR) 2>/dev/null || true; \
		printf "	✓ Application directory removed\n"; \
	fi
	@awk '{ print } /^# =+ #/ { exit }' $(CARGO) > tmp && mv tmp $(CARGO)
	@awk '/<!--MODULES-->/ { print; in_block = !in_block; next; } !in_block' $(INDEX) > tmp && mv tmp $(INDEX)
	@printf "✓ Build artifacts removed\n"

init:
	@printf "Initializing project structure...\n"
	@mkdir -p $(DIST)/external
	@touch $(DIST)/external/modules.js
	@mkdir -p $(OUT)
	@echo "pub fn register_modules(modules: &mut HashMap<String, Box<dyn Module>>) {}" > $(OUT)/module_reg.rs
	@mkdir -p $(APPDIR)/modules
	@( cd core && bun run build.ts && cd .. )
	@printf "✓ Project structure initialized\n"

ide: init install-deps build-modules modules
	@printf "Building IDE...\n"
	@cd core && bun run tauri build >/dev/null 2>&1
	@printf "✓  IDE build completed successfully!\n"
	@printf "	Binaries can be found at "
	@(cd $(OUT)/release/bundle && pwd)
	@printf "	in the following folders: "
	@ls $(OUT)/release/bundle

prod: init install-deps build-modules
	@rm -rf ./build
	@rm -rf build.zip
	@mkdir -p ./build/source
	@mkdir -p ./build/ide-bundle
	@mkdir -p ./build/empty-bundle
	@printf "Building application for production...\n"
	@printf "  Building Empty Core..."
	@$(MAKE) clean >/dev/null 2>&1
	@$(MAKE) init >/dev/null 2>&1
	@cd core/src-tauri && cargo clean >/dev/null 2>&1
	@cd core && bun run tauri build >/dev/null 2>&1
	@printf "✓\n  Empty Core build completed successfully!\n"
	@cp -r $(OUT)/release/bundle/ build/empty-bundle
	@$(MAKE) modules
	@printf "  Building IDE..."
	@cd core && bun run tauri build >/dev/null 2>&1
	@cp -r $(OUT)/release/bundle/ build/ide-bundle
	@printf "✓\n  IDE build completed successfully!\n"
	@printf "Copying files..."
	@printf "  Copying source files..."
	@$(foreach folder,$(SRC_FOLDERS),cd $(folder) && git clean -Xf . && cd .. &&) true
	@$(foreach folder,$(SRC_FOLDERS),cp -r $(folder) ./build/source &&) true
	@cp makefile ./build/source/
	@printf "✓\n  Copying readme..."
	@cp build.md ./build/README.md
	@printf "✓\nFiles copied successfully!\n"
	@printf "Zipping build folder..."
	@zip -r build.zip build >/dev/null 2>&1
	@printf "✓\nSuccessfully zipped build folder!\n"
	@printf "Build size: "
	@du -sh build.zip
	@printf "\nFinished!\n"
	@printf "Building thesis..."
	@( cd thesis && nix build >/dev/null 2>&1 && cp result/thesis.pdf ../creating-a-zero-core-modular-ide.pdf && cd - )
	@printf "✓\n Built thesis successfully!"
