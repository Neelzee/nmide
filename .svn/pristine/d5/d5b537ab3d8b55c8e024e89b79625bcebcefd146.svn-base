modules : clean
	cargo run \
		--manifest-path=../../core/cm-installer/Cargo.toml -- \
		--out=./rsm-grapher/target \
		--conf=./Modules.toml \
		--cargo=./rsm-grapher/Cargo.toml \
		--dist=./build/ \

clean :
	rm -rf ./rsm-grapher/target/debug/build/core-* && \
	rm -rf ./build/ && \
	mkdir -p ./build/