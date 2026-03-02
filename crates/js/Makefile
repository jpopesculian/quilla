CRATE = quilla_js
TARGET = wasm32-unknown-unknown
WASM_IN = ../../target/$(TARGET)/release/$(CRATE).wasm
OUT_DIR = pkg

check:
	cargo clippy

test:
	cargo test
	GECKODRIVER=$(shell which geckodriver) cargo test --target wasm32-unknown-unknown

wasm:
	rm -rf $(OUT_DIR)
	cargo build --release --target $(TARGET)
	cargo test export_bindings
	wasm-bindgen --out-dir $(OUT_DIR) $(WASM_IN)
	wasm-opt -Oz $(OUT_DIR)/$(CRATE)_bg.wasm -o $(OUT_DIR)/$(CRATE)_bg.wasm
