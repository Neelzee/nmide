modules : clean
	cargo run \
		--manifest-path=../cm-installer/Cargo.toml -- \
		--out=./rsm-grapher/target \
		--conf=./Modules.toml \
		--cargo=./rsm-grapher/Cargo.toml \
		--dist=./build/

clean :
	rm -rf ./rsm-grapher/target/debug/build/core-* && \
	rm -rf ./build/ && \
	mkdir -p ./build/ && \
	mkdir -p ./rsm-grapher/target/ && \
	touch ./build/index.html && \
	cargo run \
		--manifest-path=../cm-installer/Cargo.toml -- \
		--clean \
		--out=./rsm-grapher/target \
		--conf=./Modules.toml \
		--cargo=./rsm-grapher/Cargo.toml \
		--dist=./build/ \
		--index=./build/index.html
