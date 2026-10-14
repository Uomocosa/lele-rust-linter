# lele-rust-linter

The lele-rs convention linters and the crates they are built on.

| Crate | Role |
|---|---|
| `lele-lint` | `lele-lint` binary — enforces the lele-syntax-rs structural rules (`E0xx`) |
| `lele-bevy-lint` | `lele-bevy-lint` binary — Bevy-specific rules (`E029`/`E037`/`E038`/`E039`) |
| `atomic-delegate-macros` | `#[atomic_delegates]` proc-macro |
| `lele-snake-case` | Shared `PascalCase -> snake_case` conversion |

## Environment

Install the toolchain and the two dev tools — no nix, no container, no setup script:

```sh
rustup toolchain install nightly    # rust-toolchain.toml pins channel + components
cargo install cargo-nextest         # test runner used by `just test`
cargo install just                  # task runner
cargo binstall prek                 # hook runner (`prek install` via `just setup`)
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

`prek.toml` runs `fmt`, `clippy` and `lint` on pre-commit and `test` on pre-push, via `just`.
