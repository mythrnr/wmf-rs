# syntax=docker/dockerfile:1
FROM rust:1.82

ARG WORK_DIR="/rust"

WORKDIR ${WORK_DIR}

SHELL ["/bin/bash", "-eoux", "pipefail", "-c"]

RUN <<EOF
    #
    # Install and update packages
    #
    apt-get update
    apt-get upgrade -y

    cd /tmp || exit 1

    #
    # Cleanup
    #
    apt-get clean
    apt-get autoclean
    rm -rf \
        /var/cache/apt/* \
        /tmp/*
EOF

RUN <<EOF
    #
    # Prepare directories for Rust
    #
    mkdir -p \
        "${WORK_DIR}/cargo" \
        "${WORK_DIR}/src" \
        "${WORK_DIR}/target"

    #
    # Install tools
    #
    rustup toolchain install nightly
    rustup target add wasm32-unknown-unknown
    rustup component add \
        clippy rls rust-analysis rust-src rustfmt
    rustup component add --toolchain "nightly-$(uname -m)-unknown-linux-gnu" \
        rustfmt
    cargo install cargo-bloat cargo-machete cargo-udeps twiggy --locked

    cd /tmp || exit 1

    #
    # Install wasm-pack
    #
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

    #
    # Cleanup
    #
    rm -rf \
        /tmp/* \
        "${CARGO_HOME}/registry/"*
EOF
