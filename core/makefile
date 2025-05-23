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

install:
	@( \
		cd ../libs/javascript && \
		find . -maxdepth 1 -type d ! -path . | while read p; do \
			if [ -f "$$p/package.json" ]; then \
				echo "Installing in $$p"; \
				(cd "$$p" && bun i); \
			fi; \
		done \
	) && \
	cd ../core && echo "Installing Core JS dependencies" && bun i

clean :
	rm -rf ./src-tauri/target/debug/build/core-* && \
	rm -rf ./dist/external && \
	rm -rf /home/nmf/.local/share/no.nilsmf.uib/modules/ && \
	mkdir -p ./dist/external && \
	touch ./dist/external/modules.js && \
	mkdir -p ./src-tauri/target/ && \
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

dry :
	cargo run \
		--manifest-path=../tools/cm-installer/Cargo.toml -- \
		--out=./src-tauri/target \
		--conf=./Modules.toml \
		--modules=./modules \
		--cargo=./src-tauri/Cargo.toml \
		--dist=./dist/external \
		--index=./index.html \
		--module-dist="/home/nmf/.local/share/no.nilsmf.uib/modules/" \
		--dry-run

nmide :
	#cargo build --manifest-path=./src-tauri/Cargo.toml
