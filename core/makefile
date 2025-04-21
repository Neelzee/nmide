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
	rm -rf ./src-tauri/target/debug/build/core-*

nmide : modules
	#cargo build --manifest-path=./src-tauri/Cargo.toml
