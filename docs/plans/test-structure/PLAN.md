# PLAN: Test structure (fleet-wide)

Branch: `feature/test-structure` in `lele-rust-linter` and every fleet repo. Never merged
mid-plan (single user, single machine; main does not move). All work commits and pushes here.

Execution: Phase 0 = plain orchestrated session. Phases 1-4 = one run each of
`/lele-linter-change-rs opencode-orchestrator docs/plans/test-structure/phase-N.md`.
Orchestrator log + worker files: `<repo>/.local-run/orchestrator/2026-11-18-test-structure/`.

## Standing authorisations (override lele-lint-fix-rs "behaviour-preserving only" for this plan)
- Deleting tests the new rules forbid is allowed.
- Moving a test into the owning `*_plugin.rs` is allowed when it asserts plugin-visible
  behaviour not covered elsewhere. Expect few moves; most forbidden tests are just deleted.
- Fleet workers for phases 2-4 are SmartWorker; phase 1 workers EfficientWorker.

## Decisions (locked)
| # | Decision |
|---|---|
| D1 | Exactly one `test_usage` per file |
| D2 | No statement/line cap on `test_usage` |
| D3 | Plugin tests (`*_plugin.rs`) have free names |
| D4 | Primary item = existing atomic-file detection (table below) |
| D5 | `testing/` items must be used by >=2 test modules; cross-crate use counts |
| D6 | `LELEBEVYTEST*` in lele-bevy-lint; `LELETEST*` in lele-lint |
| D7 | Clean rename, no aliases for old codes |
| D8 | Constants are tuned in `constants.rs`, never asserted in tests |
| D9 | Honesty-based (MIR taxonomy) test rules deferred; skill guidance only |
| D10 | Extra tests in a `__methods__/` file must be `#[rstest]` (default, user may veto) |
| D11 | `just t-touched` runs only tests colocated with touched files (+ owning plugin for a touched `__bevy_systems__` file); NOT reverse dependents |

### D4: what each file tests
| File | `test_usage` shows | Other tests |
|---|---|---|
| `struct.rs` (primary type + simple inline methods) | how the type is used in real code: build it, call its methods | corner cases combining several methods; free names |
| `__methods__/<type>/<method>.rs` | that one method | `#[rstest]` corner cases for that method (D10) |
| inline simple method in `struct.rs` | no dedicated test; covered by the struct's `test_usage` | - |
| `*_plugin.rs` | the plugin added to a `TestApp` | free names (D3) |
| `__basic__/`, plain data, trivial ctor, `__bevy_systems__/` | no tests (enforced) | - |
| e2e crates (outside `src/`) | exempt | exempt |

## Rule table
| Code | Linter | Phase | Rule |
|---|---|---|---|
| LELETEST001 | lele-lint | 1 | (was LELE006) no `#[cfg(test)]` in `__basic__/` |
| LELETEST002 | lele-lint | 1->2 | (was LELE007) colocation; phase 2 adds: no `*_tests.rs`, crate-level `tests/` only in e2e crates |
| LELETEST003 | lele-lint | 1->2 | (was LELE009) `test_usage` required; phase 2: files covered by LELETEST004/LELEBEVYTEST001 need no opt-out |
| LELETEST004 | lele-lint | 2 | no tests in plain-data / trivial-constructor files |
| LELEBEVYTEST001 | lele-bevy-lint | 2 | no `#[cfg(test)]` under `__bevy_systems__/` |
| LELETEST005 | lele-lint | 3 | test module holds only test fns, `use`, `const` (no other fn/struct/enum/impl) |
| LELETEST006 | lele-lint | 3 | each `testing/` item used by >=2 test modules (D5) |
| LELETEST007 | lele-lint | 3 | `testing/` items only in `__app__/ __queries__/ __fixtures__/ __fakes__/`; test helpers only from `crate::testing::..` or a dependency's `testing` |
| LELETEST008 | lele-lint | 4 | one `test_usage` (D1); calls the primary item (D4); >=1 `assert*!` on a value derived from that call |
| LELETEST009 | lele-lint | 4 | extra tests in `__methods__/` files are `#[rstest]` (D10) |

Every new rule gets `rule_examples`; `just docs` regenerates RULES.md + `lele-lint-rules-rs`.

## Phase 0: prep
1. Branches + baseline (below). `.local-run/` gitignored everywhere.
2. This file + phase-1..4.md committed.
3. p2p repo: `.config/nextest.toml` (`[profile.worker] default-filter = "not test(/_plugin::tests::/)"`,
   slow-timeout), `just t-worker CRATE` (`rdeps`), `just t-touched` + `scripts/touched-filter.sh`
   (git diff -> colocated module filters; `__bevy_systems__` file -> owning `*_plugin.rs` tests).
   Record baseline test count + `just test` wall time. Briefs: never `-E` with `--profile worker`.

## Phases 1-4: see phase-N.md

## Done when
Every fleet repo lint-clean, gate green on the branch; p2p test count + time recorded vs baseline.

## Baseline (2026-11-18, all on main, clean)
| repo | HEAD |
|---|---|
| freenet-contract-harness | 85fa8b1 |
| freenet-libp2p-bevy-plugin | 99245f1 |
| lele-bevy-state-machine | 50bc608 |
| lele-release | 2cb4059 |
| lele-rust-function-taxonomy | d1911dd |
| lele-rust-linter | 8f850b8 |
| telegram-bot-helper | d15b2b2 |

## Log
(orchestrator appends: step, hash, worker kind+model, tokens, minutes, decisions)
