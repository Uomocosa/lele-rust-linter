# Phase 3: testing/ structure, no private helpers

Master plan: PLAN.md (D5 revised: no usage-count rule). Branch feature/test-structure.

## Linter
- LELETEST005: inside `#[cfg(test)] mod ..` only test fns (`#[test]`, `#[rstest]`,
  `#[tokio::test(..)]`, `#[rstest::rstest]`), `use`, `const`. Any other fn/struct/enum/impl/
  trait/macro_rules is an error.
- LELETEST006: items in `src/testing/` live only in `__app__/`, `__queries__/`, `__fixtures__/`,
  `__fakes__/` (plus mod.rs re-exports and the usual `__basic__`/`__traits__`/`__methods__`
  dunders inside them).

## Order (fixed)
1. Linter worker: rules + self-lint.
2. ONE p2p worker builds scaffolding in `crates/freenet-libp2p-bevy-core/src/testing/`:
   `TestApp` (MinimalPlugins, ManualDuration time, `with_plugin`, `tick(n)`),
   `find_one::<C>(..)`-style world queries, fixtures; re-home existing testing files into the
   four dunders. Follows lele-syntax-rs (atomic delegates etc.).
3. Fleet workers replace private helpers with `testing::` items. Reuse first: search the crate's
   and its dependencies' `testing/` for an existing item, and generalise one (a parameter, a
   generic) rather than adding a near-duplicate. A helper used once still moves into `testing/`
   when nothing reusable exists, or when another crate needs it. p2p: one worker per crate.
