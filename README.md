# lele-rust-function-taxonomy

Precise function-honesty taxonomy via rustc MIR: a `require = "honest"`
boundary must contain only honest functions (no hidden I/O reaching outside the
signature).

## Environment

```sh
rustup toolchain install nightly-2026-09-01   # rust-toolchain.toml pins it (+ rustc-dev)
cargo install cargo-nextest
cargo install just
cargo install --git https://github.com/Uomocosa/lele-rust-linter lele-lint
cargo binstall prek                           # hook runner (`prek install` via `just setup`)
```

The MIR driver needs the pinned nightly's `rustc-dev` component; no system
libraries beyond that.

## Tasks

```sh
just build | just clippy | just fmt | just test | just taxonomy | just lint
just setup    # one-time per clone: install the prek git hooks
just check    # run the full gate (same as CI)
just ci
```
