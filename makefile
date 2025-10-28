# Desktop
APPDIR := $(shell echo ~/.local/share/no.nilsmf.uib)
MANIFEST-PATH := tools/cm-installer/Cargo.toml
OUT := core/src-tauri/modules/src/
CONF := Modules.toml
MODULES := core/modules
CARGO := core/src-tauri/modules/Cargo.toml
DIST := core/build
INDEX := core/index.html

# Tool
TOOL-OUT := tools/module-tester/rsm-grapher/target
TOOL-CARGO := tools/module-tester/rsm-grapher/Cargo.toml
TOOL-DIST := tools/module-tester/build

# Web
WEB-APPDIR := core/src-tauri/static
WEB-MANIFEST-PATH := tools/cm-installer/Cargo.toml
WEB-CONF := Modules.toml
WEB-MODULES := core/src-tauri/static
WEB-CARGO := core/src-tauri/Cargo.toml
WEB-DIST := core/src-tauri/build
WEB-INDEX := core/src-tauri/build/index.html
WEB-TEMPLATE := core/src-tauri/template

DEV_MODE := $(if $(DEVELOPMENT),true,false)
SRC_FOLDERS := \
	core \
	libs \
	modules \
	tools

.PHONY: build-modules modules install-deps clean init ide help prod server-modules server-clean server-init

help:
	@echo "Available targets:"
	@echo "  help                 - Show this help message"
	@echo "  install-deps         - Install JavaScript libraries"
	@echo "  init                 - Initialize project structure"
	@echo "  clean                - Clean build artifacts"
	@echo "  build-modules        - Build all modules"
	@echo "  modules              - Add Modules from configuration to IDE"
	@echo "  prod                 - Full build for IDE"
	@echo "  server-init          - Initialize server project structure"
	@echo "  server-clean         - Clean server build artifacts"
	@echo "  server-modules       - Add Modules from configuration to server"

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
	@printf "Building TypeScript core...\n"
	@cd core && bun run build.ts >/dev/null 2>&1
	@printf "✓ Core build completed\n"

server-init:
	@printf "Initializing server project structure...\n"
	@mkdir -p $(WEB-DIST)
	@touch $(WEB-DIST)/modules.js
	@mkdir -p $(WEB-APPDIR)
	@mkdir -p $(OUT)
	@cp $(WEB-TEMPLATE)/index.html $(WEB-DIST)/index.html
	@( cd core/app/server && bun run build.ts )
	@printf "✓ Project structure initialized\n"

server-clean:
	@printf "Removing server build artifacts...\n"
	@rm -rf $(WEB-DIST) 2>/dev/null || true
	@printf "Removing application directory...\n";
	@rm -rf $(WEB-APPDIR) 2>/dev/null || true;
	@printf "	✓ Application directory removed\n";
	@awk '{ print } /^# =+ #/ { exit }' $(WEB-CARGO) > tmp && mv tmp $(WEB-CARGO)
	@printf "✓ Build artifacts removed\n"

server-modules: export CSS_PATH = ./static
server-modules: server-clean server-init
	@printf "Processing modules configuration...\n"
	@cargo run \
		--manifest-path=$(MANIFEST-PATH) -- \
		--conf=$(WEB-CONF) \
		--modules=$(WEB-MODULES) \
		--out=$(OUT) \
		--cargo=$(WEB-CARGO) \
		--dist=$(WEB-MODULES) \
		--index=$(WEB-INDEX) \
		--module-dist=$(WEB-APPDIR) >/dev/null 2>&1
	@printf "✓ Module configuration processed\n"
	@printf "Building server core...\n"
	@cd core/src-tauri/template && bun run build.ts >/dev/null 2>&1
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

install-module-deps:
	$(MAKE) -C modules module-build

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
	@printf "Copying files...\n"
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
	@printf "  Build size: "
	@du -sh build.zip
	@printf "\nFinished!\n"
	@printf "Building thesis..."
	@( cd thesis && nix build >/dev/null 2>&1 && cp result/thesis.pdf ../creating-a-zero-core-modular-ide.pdf && cd - )
	@printf "✓\n Built thesis successfully!"
