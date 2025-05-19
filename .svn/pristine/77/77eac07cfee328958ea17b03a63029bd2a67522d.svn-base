modules : clean
	cargo run \
		--manifest-path=../tools/cm-installer/Cargo.toml -- \
		--out=./src-tauri/target \
		--conf=./Modules.toml \
		--modules=./modules \
		--cargo=./src-tauri/Cargo.toml \
		--dist=./dist/external \
		--index=./index.html \
		--module-dist="/home/nmf/.local/share/no.nilsmf.uib/modules/" && \
	bun run build.ts
	
install :
	cd ../libs/js-utils && pwd && bun i && \
	cd ../js-client && pwd && bun i && \
	cd ../js-core-std-lib && pwd && bun i && \
	cd ../js-module-lib && pwd && bun i && \
	cd ../js-decoder-lib && pwd && bun i && \
	cd ../js-debug-module && pwd && bun i && \
	cd ../../core && pwd && bun i

clean :
	rm -rf ./src-tauri/target/debug/build/core-* && \
	rm -rf ./dist/external && \
	rm -rf /home/nmf/.local/share/no.nilsmf.uib/modules/ && \
	mkdir -p ./dist/external && \
	touch ./dist/external/modules.js && \
	touch ./src-tauri/target/module_reg.rs && \
	echo "pub fn register_modules(modules: &mut HashMap<String, Box<dyn Module>>) {}" > ./src-tauri/target/module_reg.rs && \
	mkdir -p /home/nmf/.local/share/no.nilsmf.uib/modules/ && \
	cargo run \
		--manifest-path=../tools/cm-installer/Cargo.toml -- \
		--out=./src-tauri/target \
		--clean \
		--conf=./Modules.toml \
		--modules=./modules \
		--cargo=./src-tauri/Cargo.toml \
		--dist=./dist/external \
		--index=./index.html

nmide :
	#cargo build --manifest-path=./src-tauri/Cargo.toml
