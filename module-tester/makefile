modules : clean
	cargo run \
		--manifest-path=../core/cm-installer/Cargo.toml -- \
		--out=./rsm-bundler/target \
		--conf=./Modules.toml \
		--cargo=./rsm-bundler/Cargo.toml \
		--dist=./build/ \

clean :
	rm -rf ./rsm-bundler/target/debug/build/core-* && \
	rm -rf ./build/ && \
	mkdir -p ./build/