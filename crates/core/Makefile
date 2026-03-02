define run-cargo
	cargo $(1)
	cargo $(1) --no-default-features --features libm
	cargo $(1) --no-default-features --features std
	cargo $(1) --no-default-features --features std,libm
	cargo $(1) --no-default-features --features libm,serde
	cargo $(1) --no-default-features --features std,serde
	cargo $(1) --no-default-features --features std,libm,serde
	cargo $(1) --no-default-features --features libm,thread_rng
	cargo $(1) --no-default-features --features std,thread_rng
	cargo $(1) --no-default-features --features std,libm,thread_rng
	cargo $(1) --no-default-features --features libm,serde,thread_rng
	cargo $(1) --no-default-features --features std,serde,thread_rng
	cargo $(1) --no-default-features --features std,libm,serde,thread_rng
endef

check:
	$(call run-cargo,clippy)

test:
	$(call run-cargo,test)
