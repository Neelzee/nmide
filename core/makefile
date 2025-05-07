modules : clean
	cargo run \
		--manifest-path=./cm-installer/Cargo.toml -- \
		--out=./src-tauri/target \
		--conf=./Modules.toml \
		--modules=./modules \
		--cargo=./src-tauri/Cargo.toml \
		--dist=./dist/external \
		--index=./index.html \
		--module-dist="/home/nmf/.local/share/no.nilsmf.uib/modules/"

clean :
	rm -rf ./src-tauri/target/debug/build/core-* && \
	rm -rf ./dist/external && \
	rm -rf /home/nmf/.local/share/no.nilsmf.uib/modules/ && \
	mkdir -p ./dist/external && \
	mkdir -p /home/nmf/.local/share/no.nilsmf.uib/modules/ && \
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
