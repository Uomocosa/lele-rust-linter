# lele-rust-linter

The lele-rs convention linters and the crates they are built on.

`lele-bevy-lint` is a separate binary, not a `lele-lint` flag: running it *is* "bevy mode".
It reuses `lele-lint`'s `Checker`/`Project`/`Config`/`RuleViolation` types — a second set of
checkers over the same `Project`, not a fork. Per-rule rationale lives in each rule's `why`
(`crates/lele-bevy-lint/RULES.md`, generated); there is no separate design doc to drift.

| Crate | Role |
|---|---|
| `lele-lint` | `lele-lint` binary — the lele-syntax-rs structural rules (`lele-lint --checker-list`) |
| `lele-bevy-lint` | `lele-bevy-lint` binary — Bevy-specific rules (`lele-bevy-lint --checker-list`) |
| `atomic-delegate-macros` | `#[atomic_delegates]` proc-macro |
| `lele-snake-case` | Shared `PascalCase -> snake_case` conversion |

`lele-lint-test-crates/` (outside `crates/`, never built) holds the end-to-end inputs:
`lele-lint-test-crates/<linter>/<crate>/` plus an `expected.toml` listing every `(file, code)`
pair that linter must report. To test a rule end-to-end, add or edit a crate there and its
`expected.toml` by hand.

## Dependencies and releases

This repo is the root of the lele fleet: it depends on no sibling repository,
while the others pin `atomic-delegate-macros` and install the `lele-lint` /
`lele-bevy-lint` binaries from here at release tags.

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

Install the toolchain and the two dev tools — no nix, no container, no setup script:

```sh
rustup toolchain install nightly    # rust-toolchain.toml pins channel + components
cargo install cargo-nextest         # test runner used by `just test`
cargo install just                  # task runner
cargo binstall prek                 # hook runner (`prek install` via `just setup`)
cargo install --path ../lele-rust-function-taxonomy   # honesty check (`just taxonomy`)
```

`rust-toolchain.toml` provisions the nightly channel, `rustc`/`cargo`/`clippy`/`rustfmt`.
There are **no system-library requirements** — this workspace is pure Rust.

## Tasks

```sh
just            # list recipes
just build      # cargo build --all-targets --all-features
just clippy     # cargo clippy ... -- -D warnings
just fmt        # cargo fmt -- --check
just test       # cargo nextest run --all-targets --all-features
just lint       # run lele-lint over each crate (one root lele.toml)
just taxonomy   # prove every `require = "honest"` boundary is honest (rustc MIR)
just sync-methods
just docs       # regenerate RULES.md / SKILL.md
just setup      # one-time per clone: install the prek git hooks
just check      # run the full gate (same as CI)
just ci         # build + check
```

## Git hooks

```sh
just setup      # one-time per clone: install the prek git hooks
just check      # run the full gate (same as CI)
```

`prek.toml` runs `fmt`, `clippy`, `lint` and `taxonomy` on pre-commit and `test` on pre-push, via `just`.
