# syntax=docker/dockerfile:1
FROM rust:1.88.0-slim-bookworm

ARG WORK_DIR="/work"

WORKDIR ${WORK_DIR}

SHELL ["/bin/bash", "-eoux", "pipefail", "-c"]

# Install only what the toolchain steps need on top of the base image.
# Keeping this layer lean speeds up image rebuilds when the toolchain
# below changes.
RUN <<EOF
    apt-get update
    apt-get upgrade -y
    apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        make

    apt-get clean
    apt-get autoclean
    rm -rf \
        /var/lib/apt/lists/* \
        /var/cache/apt/* \
        /tmp/*
EOF

# Toolchains and components match `rust-toolchain.toml` plus a nightly
# channel for rustfmt / cargo-udeps.
RUN <<EOF
    rustup toolchain install nightly
    rustup target add wasm32-unknown-unknown
    rustup component add clippy rustfmt rust-analyzer
    rustup component add --toolchain "nightly-$(uname -m)-unknown-linux-gnu" \
        rustfmt
EOF

# Bootstrap `cargo-binstall` from its release binary so the install
# doesn't have to satisfy a `rustc` version newer than the one pinned
# in `rust-toolchain.toml`. The remaining helpers then come from
# pre-built GitHub releases instead of slow source builds. The two
# `cargo install` lines at the end are for tools that don't ship
# binstall-friendly artifacts at the moment.
RUN <<EOF
    curl -L --proto '=https' --tlsv1.2 -sSf \
        https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh \
        | bash

    cargo binstall -y \
        cargo-machete \
        cargo-udeps \
        wasm-bindgen-cli \
        wasm-opt \
        wasm-pack

    cargo install --locked cargo-bloat twiggy

    rm -rf \
        /tmp/* \
        "${CARGO_HOME}/registry/"*
EOF

# Drop interactive users into bash by default so `docker compose run`
# behaves as an honest dev shell.
CMD ["/bin/bash"]
