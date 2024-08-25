ifndef VERBOSE
MAKEFLAGS += --silent
endif

.PHONY: check
check:
	cargo check --workspace --all-targets --all-features

.PHONY: clean
clean:
	cargo clean

.PHONY: doc
doc:
	cargo doc --open --workspace

.PHONY: fix
fix:
	cargo fix --allow-dirty --allow-staged

.PHONY: fmt
fmt:
	cargo +nightly fmt --all

.PHONY: lint
lint:
	cargo clippy --workspace --all-targets --all-features \
		-- --no-deps -D warnings

.PHONY: spell-check
spell-check:
	docker pull ghcr.io/streetsidesoftware/cspell:latest > /dev/null \
	&& docker run --rm \
		-v $(shell pwd):/workdir \
		ghcr.io/streetsidesoftware/cspell:latest \
			--config .vscode/cspell.json "**"

.PHONY: test
test:
	cargo test --workspace --all-targets

.PHONY: udeps
udeps:
	cargo machete
	cargo +nightly udeps --all-targets
