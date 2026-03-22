ifndef VERBOSE
MAKEFLAGS += --silent
endif

.PHONY: ci-suite
ci-suite: spell-check fix fmt lint udeps test

.PHONY: check
check:
	cargo check --workspace --all-targets --all-features

.PHONY: clean
clean:
	cargo clean

.PHONY: doc
doc:
	cargo doc --open --workspace --no-deps

.PHONY: fix
fix:
	cargo fix --allow-dirty --allow-staged

.PHONY: fmt
fmt:
	cargo +nightly fmt --all

.PHONY: install-tools
install-tools:
	curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
	cargo binstall -y cargo-machete
	cargo binstall -y cargo-udeps
	cargo binstall -y wasm-pack

.PHONY: lint
lint:
	cargo clippy --workspace --all-targets --all-features \
		-- --no-deps -D warnings

.PHONY: release
release:
	if [ "$(version)" = "" ]; then \
		echo "release version is required."; \
		exit 1; \
	fi \
	&& git tag $(version) \
	&& git push origin $(version)

.PHONY: serve
serve: wasm
	yarn
	yarn run serve -p 8080 wasm/dist/

.PHONY: spell-check
spell-check:
	docker run --pull always --rm -v "$(shell pwd):/workdir" \
		ghcr.io/streetsidesoftware/cspell:latest \
			--config .vscode/cspell.json "**"

.PHONY: test
test:
	cargo test --workspace --all-targets

.PHONY: udeps
udeps:
	cargo machete
	cargo +nightly udeps --all-targets

.PHONY: wasm
wasm:
	cd wasm && wasm-pack build --out-dir dist --target web
