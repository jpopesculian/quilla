define run-make
	make -C core $(1)
	make -C js $(1)
	make -C lang $(1)
endef

check:
	$(call run-make,check)

test:
	$(call run-make,test)

wasm:
	make -C js wasm
