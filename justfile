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

# Show which binary each tool resolves to and its version. A `<fleet>/bin` shim
# (written by `lele-release setup`) builds the sibling checkout; set
# LELE_USE_INSTALLED=1 to use the installed binary instead.
tools:
    @for tool in lele-function-taxonomy; do printf '%-24s %s  %s\n' "$tool" "$(command -v "$tool")" "$("$tool" --version)"; done

# Regenerate every fixture's committed Cargo.lock after changing its manifest
# (fixtures are hermetic: the lock pins the exact dependency revisions).
fixtures-lock:
    for manifest in tests/fixtures/*/Cargo.toml; do cargo metadata --manifest-path "$manifest" --format-version 1 > /dev/null; done

setup:
    prek install -t pre-commit -t pre-push

check:
    prek run --all-files

ci: build check
