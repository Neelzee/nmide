addModules : cleanCore
	cargo run \
		--manifest-path=./src-tauri/core-cm-installer/Cargo.toml -- \
		--out=./src-tauri/target \
		--conf=./modules/Modules.toml \
		--cargo=./src-tauri/Cargo.toml

cleanCore :
	rm -rf ./src-tauri/target/debug/build/core-*

nmide : addModules
	cargo build --manifest-path=./src-tauri/Cargo.toml
