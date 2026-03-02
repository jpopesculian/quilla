define run-cargo
	cargo $(1)
	cargo $(1) --no-default-features --features libm
	cargo $(1) --no-default-features --features std
	cargo $(1) --no-default-features --features std,libm
endef

check:
	$(call run-cargo,clippy)

test:
	$(call run-cargo,test)
