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

setup:
    prek install -t pre-commit -t pre-push

check:
    prek run --all-files

ci: build check
