# lele-lint-test-crates

End-to-end inputs for the linters: `lele-lint/<crate>` is linted by `crates/lele-lint/tests/e2e.rs`,
`lele-bevy-lint/<crate>` by `crates/lele-bevy-lint/tests/e2e.rs`. These crates are never built.

Each crate has an `expected.toml` listing every `(file, code)` pair the linter must report — no
more, no less. Edit it by hand; a clean crate has `violations = []`.
