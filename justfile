set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

# Toolchain: rust-toolchain.toml (rustup provisions it).
# Extra dev tool: cargo-nextest  (`cargo install cargo-nextest`).

# --- checks ---------------------------------------------------------------
build:
    cargo build --all-targets --all-features

clippy:
    cargo clippy --all-targets --all-features -- -D warnings

fmt:
    cargo fmt -- --check

test:
    cargo nextest run --all-targets --all-features

# --- lele lint (one root lele.toml, one pass per crate) --------------------
lint:
    cargo run --quiet -p lele-lint -- --config lele.toml crates/lele-lint
    cargo run --quiet -p lele-lint -- --config lele.toml crates/lele-bevy-lint
    cargo run --quiet -p lele-lint -- --config lele.toml crates/atomic-delegate-macros
    cargo run --quiet -p lele-lint -- --config lele.toml crates/lele-snake-case

sync-methods:
    cargo run --quiet -p lele-lint -- --sync-methods crates/lele-lint
    cargo run --quiet -p lele-lint -- --sync-methods crates/lele-bevy-lint

# --- git hooks -------------------------------------------------------------
hooks:
    git config core.hooksPath .githooks
    chmod +x .githooks/*

# --- aggregate -------------------------------------------------------------
ci: build fmt clippy lint test
