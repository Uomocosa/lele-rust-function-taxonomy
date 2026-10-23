set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

# Toolchain: rust-toolchain.toml (pinned nightly + rustc-dev for the MIR driver).
# Extra tools: cargo-nextest, just, and `lele-lint`.

build:
    cargo build --all-targets --all-features

clippy:
    cargo clippy --all-targets --all-features -- -D warnings

fmt:
    cargo fmt -- --check

test:
    cargo nextest run --all-targets --all-features

taxonomy:
    cargo run --bin lele-function-taxonomy -- --manifest-path ./Cargo.toml

lint:
    lele-lint .

# --- tool install (reproducible) ---------------------------------------------
# Install both binaries from this checkout, honouring Cargo.lock. The toolchain
# comes from rust-toolchain.toml (nightly + rustc-dev) because the driver is
# built with rustc_private.
install:
    cargo install --locked --force --path .

# Fail fast when the installed binary is not the version this repo expects.
tools:
    test "$(lele-function-taxonomy --version)" = "lele-function-taxonomy 0.0.2"

setup:
    prek install -t pre-commit -t pre-push

check:
    prek run --all-files

ci: build check
