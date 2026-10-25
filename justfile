set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

# Toolchain: rust-toolchain.toml (rustup provisions it).
# Extra dev tools: cargo-nextest (`cargo install cargo-nextest`),
# lele-function-taxonomy (`cargo install --path ../lele-rust-function-taxonomy`).

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

# honesty half of every `require = "honest"` boundary (rustc MIR)
taxonomy:
    lele-function-taxonomy --manifest-path ./Cargo.toml

sync-methods:
    cargo run --quiet -p lele-lint -- --sync-methods crates/lele-lint
    cargo run --quiet -p lele-lint -- --sync-methods crates/lele-bevy-lint

# regenerate RULES.md / SKILL.md (the rules_md / skill_md tests fail while stale)
docs: skill
    cargo run --quiet -p lele-lint -- --rules-md > crates/lele-lint/RULES.md
    cargo run --quiet -p lele-bevy-lint -- --emit rules-md > crates/lele-bevy-lint/RULES.md
    cargo run --quiet -p lele-bevy-lint -- --emit skill-md > .opencode/skills/bevy-ui-preview/SKILL.md

# regenerate the lele-lint-rules-rs agent skill (skills/lele-lint-rules-rs/SKILL.md)
skill:
    mkdir -p skills/lele-lint-rules-rs
    cargo run --quiet -p lele-lint -- --skill-md > skills/lele-lint-rules-rs/SKILL.md

# --- tool install (reproducible) ---------------------------------------------
# Install both binaries from this checkout. Use this instead of a bare
# `cargo install --path`, which re-resolves dependencies to the newest
# compatible versions and silently installs a different binary.
install:
    cargo install --locked --force --path crates/lele-lint
    cargo install --locked --force --path crates/lele-bevy-lint

# Show which binary each tool resolves to and its version. A `<fleet>/bin` shim
# (written by `lele-release setup`) builds the sibling checkout; set
# LELE_USE_INSTALLED=1 to use the installed binary instead.
tools:
    @for tool in lele-lint lele-bevy-lint; do printf '%-24s %s  %s\n' "$tool" "$(command -v "$tool")" "$("$tool" --version)"; done

# --- prek hooks (commit/push gate; tasks above) -------------------------------
setup:
    prek install -t pre-commit -t pre-push

check:
    prek run --all-files

# --- aggregate -------------------------------------------------------------
ci: build check
