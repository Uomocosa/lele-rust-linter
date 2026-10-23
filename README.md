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
