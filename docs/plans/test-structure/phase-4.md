# Phase 4: test shape

Master plan: PLAN.md (D1, D2, D4, D10). Branch feature/test-structure.

## Linter
- LELETEST008: exactly one `fn test_usage` per test module; its body calls the file's primary
  item (atomic-file detection: for a type file, constructs the type or calls one of its methods;
  for a `__methods__` file, calls that method; for a plugin file, adds the plugin); contains
  >=1 `assert*!` whose args mention a binding produced by that call. No statement cap (D2).
- LELETEST009: in `__methods__/` files every test other than `test_usage` is `#[rstest]`.

## Fleet
- Fix shapes. In struct files, a test exercising a single inline method alone is folded into
  `test_usage` or deleted (D4).

## Skill
- `lele-syntax-rs`: "How to write tests here" section: D4 table, D8, dishonest code
  (split core/shell -> fake via testing/__fakes__ -> else e2e), rstest for honest corner cases.
