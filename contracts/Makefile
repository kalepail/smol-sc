# CA3SPLLDBCOVZDDFAXNDNDBWH5E3ULRX5AL2MVQWOCGLJO7IGO5YHE7J

default: build

all: test

test: build
	cargo test

build:
	stellar contract build
	stellar contract optimize --wasm target/wasm32-unknown-unknown/release/smol_sc.wasm
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

deploy: build
	stellar contract deploy \
		--wasm target/wasm32-unknown-unknown/release/smol_sc.optimized.wasm \
		--network testnet \
		--source default \
		-- \
		--admin default \
		--fee-sac CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC \
		--fee-address default \
		--color-claim-fee 2500000000 \
		--glyph-mine-fee 10000000 \
		--color-owner-royalty-rate 2 \
		--glyph-author-royalty-rate 5

bindings: build
	stellar contract bindings typescript \
	--wasm target/wasm32-unknown-unknown/release/smol_sc.wasm \
	--output-dir ../smol-sc-sdk \
	--overwrite

fmt:
	cargo fmt --all

clean:
	cargo clean
