modules : clean
	cargo run \
		--manifest-path=./cm-installer/Cargo.toml -- \
		--out=./src-tauri/target \
		--conf=./Modules.toml \
		--modules=./modules \
		--cargo=./src-tauri/Cargo.toml \
		--dist=./dist/external \
		--index=./index.html

clean :
	rm -rf ./src-tauri/target/debug/build/core-* && \
	rm -rf ./dist/external && \
	mkdir -p ./dist/external && \
	cargo run \
		--manifest-path=./cm-installer/Cargo.toml -- \
		--out=./src-tauri/target \
		--clean \
		--conf=./Modules.toml \
		--modules=./modules \
		--cargo=./src-tauri/Cargo.toml \
		--dist=./dist/external \
		--index=./index.html

nmide :
	#cargo build --manifest-path=./src-tauri/Cargo.toml
