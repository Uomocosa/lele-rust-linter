# PLAN: Test structure (fleet-wide)

> Resuming? Read HANDOFF.md first.

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
| D5 | (revised 2026-11-19, user) REMOVED: no usage-count rule on `testing/` items (the linter sees one repo, and some `testing/` modules exist for other repos). Private helpers move into the crate's `testing/` dunders even when used once; reuse an existing `testing/` item first (skill guidance, lele-syntax-rs RATIONALE §17 / SMELLS S9) |
| D6 | `LELEBEVYTEST*` in lele-bevy-lint; `LELETEST*` in lele-lint |
| D7 | Clean rename, no aliases for old codes |
| D8 | Constants are tuned in `constants.rs`, never asserted in tests |
| D9 | Honesty-based (MIR taxonomy) test rules deferred; skill guidance only |
| D10 | Extra tests in a `__methods__/` file must be `#[rstest]` (default, user may veto) |
| D12 | (user, 2026-11-18) LELEBEVYTEST001 allows a test module in a `__bevy_systems__/` file only if it holds nothing but preview tests (`*_ui_png_preview` / mp4 previews, as the UI rules detect them); previews are the visual usage of a UI system and stay colocated. UI rules unchanged |
| D13 | (user, 2026-11-19) Preview authoring tools (`scene::Clock`, `Scene::hover::<C>`) are public API of `lele-bevy-preview`, not `testing/` items |
| D14 | (user, 2026-11-19) `PeekedRoomFriends` stays: read by examples/starboard `draw_friend_highlights` (the phase-2 "dead code" report missed examples) |
| D15 | (user, 2026-11-19) `examples/` stay outside lint scope for now |
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
| LELETEST006 | lele-lint | 3 | (was 007) `testing/` items only in `__app__/ __queries__/ __fixtures__/ __fakes__/`; test helpers only from `crate::testing::..` or a dependency's `testing` |
| LELETEST007 | lele-lint | 4 | (was 008) one `test_usage` (D1); calls the primary item (D4); >=1 `assert*!` on a value derived from that call |
| LELETEST008 | lele-lint | 4 | (was 009) extra tests in `__methods__/` files are `#[rstest]` (D10) |

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
- P0: branches + plan (9ebe8c7, 84ac934), pushed. Orchestrator.
- P1 linter: 10ea29e LELETEST001-003 rename, pushed. EfficientWorker muse-spark-1.3-contributor#low. Fleet check: 0 errors on all 24 targets.
- P1 skills: ~/.config/opencode 86cbea6 (lele-ponytail-bridge-rs LELETEST003 + __methods__ paths).
- P2 A: 32fc336 LELEBEVYTEST001 (+D12 preview exemption, user decision). EfficientWorker muse-spark, 2 turns (1 stop on conflict).
- P0 nextest: p2p work done by SmartWorker (2 turns), commit blocked by new rules in prek; kept as .local-run patch, re-applied uncommitted, lands with p2p phase-2 commit. Baseline 1554 tests / 4m03; t-worker lobby-room 775 tests / 1m30 (was 886 / 2m40). Decision: default slow-timeout 60s reporting only; worker profile skips *_plugin and *_preview tests.
- P2 B: a301021 LELETEST004 (own is_plain_data predicate; is_type_only ignores trait impls). EfficientWorker.
- P2 C: be02dbf LELETEST002 *_tests.rs + e2e_test_dirs config key. EfficientWorker.
- P2 fleet check (error lines): harness 1x003; p2p core 1x002 17x003 8x004 143xBEVYTEST001, lobby-room 1x002 11x003 3x004 901xBEVYTEST001, contract/shared/common 1/2/1x003; state-machine 3x003 34xBEVYTEST001; release 1x002 1x003; taxonomy 1x002 2x003; telegram helper 1x002, preview 1x002 2x003 6xBEVYTEST001. Compiles: unaffected (no macro change).
- P2 fleet plan: p2p = 13 sequential SmartWorker jobs (~14 files, lists in p2p .local-run job-*), uncommitted until whole repo clean (pre-commit lints all crates), then commit per area. Small repos: one SmartWorker job each, commit+push.
- P2 fix: ba0a247 LELETEST003 needs no test_usage in bevy systems (job B missed it). EfficientWorker.
- P2 fleet small repos (SmartWorker each): state-machine 0ea1ff9 (orchestrator committed worker diff; 23->19 tests, 1 moved to plugin), telegram 7d00185 (center_roots preview now shows centred/centred instead of before/after: open for user), taxonomy 0b14b00, harness a3c4680, release 41be347 (fixture moved out of tests/).
- P2 p2p job 1 core-misc done (uncommitted): 17 opt-outs, 10 tests deleted, spectator_fake_network moved into spectator_plugin tests (+~30 helpers: phase 3). Orchestrator fixed touched-filter.sh nested __basic__ path. ShortPeerId truncation case added to job 2.
- P2 p2p jobs 2-8 done (core-sys-00/01, lobby-misc, lobby-disc-00..03), uncommitted. Recurring worker error: "flagging" uncovered logic instead of moving it (4 send-backs); brief tightened.
- t-touched slowness (user spotted): touched lib.rs mapped to whole package (~580 tests, 64s floor = mp4 previews) + --workspace --all-targets compiled examples/e2e (~45s) + previews posted to Telegram. Fixed (orchestrator): crate root -> root tests:: only, build only touched packages --lib --bins, TELEGRAM_* blanked. 81 tests / 23s for the whole accumulated diff (was ~110-150s per run).
- Pending for p2p commit: clippy too_many_lines in disc-01/02 preview fns (inlined helpers) -> cleanup job before commit.
- Open for user: PeekedRoomFriends (written by sync_peeked_room_friends) has no consumer: likely dead after the starboard friends-on-board change (disc-05 report).
- Open for user: rendezvous_preview_dial's me: Option<&PeerId> None branch unreachable (both callers pass Some) - make non-optional? (lobby-sys-00 report).
- Open for user: previews that lost content because D12 forbids test-only helpers/resources in system files: telegram center_roots (before/after now centred/centred), drive_roll_machines mp4 (widget-vs-plain-text pair gone), sync_room_friend_segment pulse (timing source changed, frames reportedly same). Phase 3 testing/__fixtures__ could restore them.
- P2 p2p all 13 jobs done (uncommitted). lobby-room lint-clean; core 9x LELETEST003 opt-outs folded into p2-clippy job. Open for user: hover previews (refresh_roster_tooltip, refresh_spectator_tooltip) now use PointerCaptureMap::capture with a const SYNTHETIC_HIT instead of a test-only HoverTarget resource - bless or replace with a testing/ fixture in phase 3.
- P2 p2p: clippy cleanup job (too_many_lines in 2 previews via consts, 9 core opt-outs); orchestrator removed 4 missed opt-outs in contract/shared_types/common_types. Full suite: 1208 passed / 31 skipped (baseline 1554 tests); test time 269s vs 242s baseline (previews + moved multi-swarm plugin tests dominate; machine loaded).
- P2 p2p commit 1341a60 (single commit: prek stashes unstaged changes and lints the staged tree, so per-area commits cannot pass while intermediate states fail lint). PHASE 2 DONE fleet-wide.
- Lesson for phases 3/4: plan fleet commits as one commit per repo, or order jobs so each staged state is lint-clean.
- 2026-11-19 open items decided with user: D5 removed (rules renumbered: phase-3 LELETEST006 = testing/ dunders; phase-4 LELETEST007/008), D13-D15; rendezvous_preview_dial -> non-optional `me` + moved to peek/rendezvous_preview_dial.rs; previews restored via lele-bevy-preview `scene::Clock` + `Scene::hover`; clippy too_many_lines left as is. Work dir: lele-rust-linter/.local-run/orchestrator/2026-11-19-open-items/.
- 2026-11-19 executed: telegram-bot-helper 1c64193 (lele-bevy-preview scene::Clock + Scene::hover; center_roots before/after restored, PNGs pixel-identical to pre-phase-2), p2p 3a746c2 (drive_roll_machines real row 9->10->11 + plain-text reference; pulse + both tooltip previews on Clock/hover, tooltip PNGs pixel-identical; rendezvous_preview_dial `me: &PeerId`). SmartWorker deepseek-v4.1-flash#low, ~$0.12 total. Skills dc6309f (testing/ reuse-first: RATIONALE §17, SMELLS S9, lele-linter-change-rs checks).
- Open for user: rendezvous_preview_dial cannot leave __bevy_systems__/ as a free fn (LELE034 no_free_functions; owner types PeerId/RoomMember/Room are foreign). Options: method on shared_types Room, newtype owner, stay (untested), entry_points exemption. Move + 5 restored tests saved in p2p .local-run/orchestrator/2026-11-19-open-items/change4-moved.patch.
- 2026-11-19 rendezvous_preview_dial: user chose `Room::rendezvous_preview_dial(&self, me)` in lobby-room shared_types; p2p e31f055 (5 tests restored). EfficientWorker muse-spark-1.3-contributor#low, ~$0.01. Open question closed.
