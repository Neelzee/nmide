# Desktop
APPDIR := $(shell echo ~/.local/share/no.nilsmf.uib)
MANIFEST-PATH := src-tauri/tools/cm-installer/Cargo.toml
OUT := src-tauri/core_modules/src/
CONF := Modules.toml
MODULES := src-tauri/modules
CARGO := src-tauri/core_modules/Cargo.toml
DIST := build
INDEX := index.html

# Tool
TOOL-OUT := src-tauri/tools/module-tester/rsm-grapher/target
TOOL-CARGO := src-tauri/tools/module-tester/rsm-grapher/Cargo.toml
TOOL-DIST := src-tauri/tools/module-tester/build

# Web
WEB-APPDIR := src-tauri/static
WEB-MANIFEST-PATH := src-tauri/tools/cm-installer/Cargo.toml
WEB-CONF := Modules.toml
WEB-MODULES := src-tauri/static
WEB-CARGO := src-tauri/Cargo.toml
WEB-DIST := src-tauri/build
WEB-INDEX := src-tauri/build/index.html
WEB-TEMPLATE := src-tauri/template

OK := ✓
ERR := ✗

DEV_MODE := $(if $(DEVELOPMENT),true,false)

.PHONY: build-modules modules install-deps clean init ide help prod server-modules server-clean server-init

help:
	@echo "Available targets:"
	@echo "  help                 - Show this help message"
	@echo "  install-deps         - Install JavaScript libraries"
	@echo "  init                 - Initialize project structure"
	@echo "  clean                - Clean build artifacts"
	@echo "  build-modules        - Build all modules"
	@echo "  modules              - Add Modules from configuration to IDE"

build-modules:
	@echo "Building modules..."
	@$(MAKE) -C src-tauri/modules module-build
	@echo "Modules built successfully $(OK)"

modules: clean init
	@echo "Processing modules configuration..."
	@cargo run \
		--manifest-path=$(MANIFEST-PATH) -- \
		--out=$(OUT) \
		--conf=$(CONF) \
		--modules=$(MODULES) \
		--cargo=$(CARGO) \
		--dist=$(DIST)/external \
		--index=$(INDEX) \
		--module-dist=$(APPDIR)/modules
	@echo "Module configuration processed $(OK)"
	@echo "Building TypeScript core..."
	@bun run build.ts
	@echo "Core build completed $(OK)"

install-deps:
	@echo "Linking JavaScript libraries..."
	@( \
		cd ./src-tauri/libs/javascript && \
		find . -maxdepth 1 -type d ! -path . | while read p; do \
			if [ -f "$$p/package.json" ]; then \
				echo "- Linking $$p... "; \
				if (cd "$$p" && bun link); then \
					echo "$(OK)"; \
				else \
					echo "$(ERR)"; \
				fi; \
			fi; \
		done && \
		find . -maxdepth 1 -type d ! -path . | while read p; do \
			if [ -f "$$p/package.json" ]; then \
				printf "  Installing $$p... "; \
				if (cd "$$p" && bun i); then \
					echo "$(OK)"; \
				else \
					echo "$(ERR)"; \
				fi; \
			fi; \
		done \
	)
	@echo "Installing core dependencies..."
	@bun i
	@echo "All dependencies installed $(OK)"

clean:
	@echo "Removing build artifacts..."
	@rm -rf $(OUT)/debug/build/core-* || true
	@rm -rf $(DIST)/external || true
	@if [ "$(DEV_MODE)" = "false" ] && [ -d "$(APPDIR)" ]; then \
		echo "WARNING: About to delete application directory: $(APPDIR)"; \
		echo "Continue? [y/N]"; \
		read -r confirm; \
		if [ "$$confirm" = "y" ] || [ "$$confirm" = "Y" ]; then \
			echo "	Removing application directory..."; \
			rm -rf $(APPDIR); \
			echo "	Application directory removed $(OK)"; \
		else \
			echo "Skipping application directory removal"; \
		fi; \
	else \
		echo "Removing application directory..."; \
		rm -rf $(APPDIR) || true; \
		echo "	Application directory removed $(OK)"; \
	fi
	@awk '{ print } /^# =+ #/ { exit }' $(CARGO) > tmp && mv tmp $(CARGO)
	@awk '/<!--MODULES-->/ { print; in_block = !in_block; next; } !in_block' $(INDEX) > tmp && mv tmp $(INDEX)
	@echo "Build artifacts removed $(OK)"

init:
	@echo "Initializing project structure..."
	@mkdir -p $(DIST)/external
	@touch $(DIST)/external/modules.js
	@mkdir -p $(OUT)
	@echo "pub fn register_modules(modules: &mut HashMap<String, Box<dyn Module>>) {}" > $(OUT)/module_reg.rs
	@mkdir -p $(APPDIR)/modules
	@bun run build.ts
	@echo "Project structure initialized $(OK)"
