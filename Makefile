ifndef VERBOSE
MAKEFLAGS += --silent
endif

.PHONY: ci-suite
ci-suite: spell-check fix fmt lint doc-check udeps wasm wasm-minimal test

.PHONY: check
check:
	cargo check --workspace --all-targets --all-features

.PHONY: clean
clean:
	git clean -fdX

.PHONY: doc
doc:
	cargo doc --open --workspace --no-deps

.PHONY: doc-check
doc-check:
	RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps \
		--all-features

.PHONY: docker-build
docker-build:
	docker compose -f docker/compose.yaml --progress plain build

.PHONY: docker-clean
docker-clean:
	docker compose -f docker/compose.yaml --progress plain down \
		--volumes --remove-orphans

.PHONY: docker-dev
docker-dev:
	docker compose -f docker/compose.yaml --progress plain run --rm dev

.PHONY: fix
fix:
	cargo fix --allow-dirty --allow-staged

.PHONY: fmt
fmt:
	cargo +nightly fmt --all

.PHONY: install-tools
install-tools:
	curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
	cargo binstall -y \
		cargo-machete \
		cargo-release \
		cargo-udeps \
		wasm-bindgen-cli \
		wasm-opt \
		wasm-pack

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
	&& if [ "$$(git branch --show-current)" != "master" ]; then \
		echo "release branches must start from master."; \
		exit 1; \
	fi \
	&& git switch -c "release/$(version)" \
	&& cargo release version $(version) --execute \
	&& cargo update --workspace \
	&& echo "Version bumped." \
	&& echo "Commit and open a PR; merging to master triggers the release."

.PHONY: serve
serve: wasm
	yarn
	yarn run serve -p 8080 wasm/dist/

.PHONY: spell-check
spell-check:
	docker run --pull always --rm -v "$(shell pwd):/workdir" \
		ghcr.io/streetsidesoftware/cspell:latest \
			--config .vscode/cspell.json "**"

# `--all-targets` does not include doctests, so they are run as a
# separate step to keep the doc examples compiling. `wmf-wasm` is
# excluded because doctests cannot run against a cdylib-only crate.
.PHONY: test
test:
	cargo test --workspace --all-targets
	cargo test --workspace --exclude wmf-wasm --doc

.PHONY: udeps
udeps:
	cargo machete
	cargo +nightly udeps --all-targets

.PHONY: wasm
wasm:
	cd wasm && wasm-pack build \
		--out-dir dist \
		--release \
		--target web \
		-- --locked

# Minimal build with the `tracing` feature stripped. Drops the
# `tracing-wasm` dependency entirely, which yields a noticeably smaller
# bundle for consumers that do not need log output. `console_error_panic_hook`
# is kept so panics still surface in the browser console.
.PHONY: wasm-minimal
wasm-minimal:
	cd wasm && wasm-pack build \
		--out-dir dist-minimal \
		--release \
		--target web \
		-- --locked --no-default-features --features console_error_panic_hook
