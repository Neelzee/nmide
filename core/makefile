APPDIR := $(shell echo ~/.local/share/no.nilsmf.uib)
MODULES := $(shell echo ../modules)
CONFIG := $(shell echo ../Modules.toml)

modules :
	$(MAKE) -C .. modules

clean :
	$(MAKE) -C .. clean

init :
	$(MAKE) -C .. init

.PHONY: modules clean init server

server : export DIST_DIR = ./build/
server :
	@printf "Installing modules...\n"
	@(cd src-tauri && cargo run --features server -- install)
	@printf "$(APPDIR)"
