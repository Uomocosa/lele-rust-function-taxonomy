# lele-rust-function-taxonomy

Precise function-honesty taxonomy via rustc MIR: a `require = "honest"`
boundary must contain only honest functions (no hidden I/O reaching outside the
signature).

## Dependencies and releases

This crate depends on the sibling repository `lele-rust-linter`
(`atomic-delegate-macros`), and is consumed as two binaries
(`lele-function-taxonomy`, `lele-taxonomy-driver`). Committed manifests pin that
dependency at a release tag:

    atomic-delegate-macros = { git = "https://github.com/Uomocosa/lele-rust-linter", tag = "v0.0.3" }

A git content filter (`lele-pins`) rewrites those pins to sibling `path =`
dependencies in your working tree, so an edit in a dependency is picked up by the
next `cargo build` here with no release and no command. Install it once per
machine:

    cd ../lele-release && just setup

Never hand-edit a pin. From the fleet root, `just release <patch|minor|major>`
bumps every changed repo, re-pins its dependents, tags `vX.Y.Z` and pushes; see
`lele-release/README.md` for `status`, `--only`/`--exclude` and `--dry-run`. A
plain clone without the filter resolves the committed tags, which is exactly what
CI sees.

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
