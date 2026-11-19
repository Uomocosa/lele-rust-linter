# HANDOFF: test-structure refactor (state at end of 2026-11-18)

Read this first, then PLAN.md (decisions D1-D12, rule table, log) and phase-3.md / phase-4.md.
Run as orchestrator with `/opencode-orchestrator` + the `lele-linter-change-rs` flow.

## Where we are
- Branch `feature/test-structure` in all 7 fleet repos; every tree clean; all pushed
  (p2p `1341a60` push was running at handoff: check `git -C freenet-libp2p-bevy-plugin status -sb`
  shows no `[ahead`; if it is ahead, `git push` it - pre-push runs the full suite, ~6 min).
- Phase 0 (nextest tooling), Phase 1 (rename LELE006/007/009 -> LELETEST001-003) and Phase 2
  (LELETEST002 ext, LELETEST004, LELEBEVYTEST001, D12 preview exemption) are DONE fleet-wide.
  Every lint target of every repo is clean.
- `~/.config/opencode` has 1 unpushed commit `86cbea6` (bridge skill code rename) and the user's
  own uncommitted changes (orchestrator-core, ui-sketch-rs): never touch those.
- p2p numbers: 1554 tests before -> 1208 after (all green, 31 skipped). Suite ~269 s test time,
  dominated by mp4/scene previews and multi-swarm plugin tests.

## Next: Phase 3 (phase-3.md)
Order is fixed: (1) linter worker: LELETEST005 (test module holds only test fns + use + const),
LELETEST006 (testing/ items only in __app__/ __queries__/ __fixtures__/ __fakes__/). No usage-count
rule (D5 removed 2026-11-19): reuse existing testing/ items first, single-use helpers still go in
testing/ (skill: lele-syntax-rs RATIONALE §17, SMELLS S9). (2) ONE p2p worker builds the testing/
scaffolding in freenet-libp2p-bevy-core/src/testing/ (TestApp, world queries, fixtures, re-home the
existing testing files). (3) fleet workers replace private helpers.
Known phase-3 targets: spectator_plugin.rs tests (~30 helpers moved in from the old
spectator_fake_network.rs), p2p_plugin.rs tests (8 helpers + Node/TestPayload from
runner_config_tests), plus the ~1,186 test-module helpers counted at the start (many now gone with
deleted system tests - recount first).
Do a fleet count (step 3 of lele-linter-change-rs) BEFORE splitting work; Phase 3 is likely the
biggest phase in p2p.

Then Phase 4 (phase-4.md): LELETEST007 (one test_usage per file calling the primary item) and
LELETEST008 (extra tests in __methods__/ are #[rstest]). Pre-existing second test modules exist
(`build_plugin_tests` in core p2p_plugin.rs, lobby discovery/plugin.rs, ui/direct_dial/
friends_ui_plugin.rs; `png_tests` in layout_stream_panel.rs) - LELETEST007 must decide on them.

## Open decisions for the USER (ask before acting)
Items 1-5 of 2026-11-18 decided on 2026-11-19 (PLAN.md D5 revised, D13-D15, log). Remaining:
6. D10 (rstest-only extras in __methods__/) was an orchestrator default; user never vetoed.

## Tooling (local, gitignored) - p2p `.local-run/orchestrator/2026-11-18-test-structure/`
- `worker-env.md` (copy also in lele-rust-linter/.local-run/...): every brief starts "read it first".
- `p2-fleet-brief.md` + `launch-job.sh JOB FILELIST [EXTRA_NOTE]`: p2p job template/launcher.
- `verify-job.sh JOB FILELIST`: lint-left-in-job-files + out-of-leash files + report head/questions.
- lele-rust-linter `.local-run/.../p2-small-brief.md` + `launch-small.sh REPO "ERRORS"`: small repos.
- Reports of every job are there (`*-report.md`), plus logs (`*.jsonl`).

## Gotchas learned (do not repeat)
- prek pre-commit STASHES unstaged changes and lints only the staged tree: per-area commits fail
  while other files are still red. Plan one commit per repo, or order jobs so each staged state is
  lint-clean. Failed commits leave files staged (the next commit swallows them).
- The `bin/` shims build the sibling linter checkout: a new rule is live fleet-wide instantly and
  blocks every fleet commit until that repo is fixed. Use `LELE_USE_INSTALLED=1` to count with the
  last committed linter while a linter worker is mid-edit.
- Workers (deepseek SmartWorker) repeatedly "flag" uncovered logic instead of moving it; the brief
  now says MOVE explicitly. Still check every report's open questions; expect ~1 send-back in 3.
- `t-touched` was slow because a touched lib.rs meant "whole crate" and builds were
  --workspace --all-targets, and previews posted to Telegram. Fixed (crate root -> root tests::,
  -p touched packages --lib --bins, TELEGRAM_* blanked). Keep TELEGRAM_* blank for every worker run.
- `just test` takes no args: use NEXTEST_NO_FAIL_FAST=1, not `--no-fail-fast`.
- Launch workers with the Bash tool's run_in_background, never `& disown` (no exit notification).
  Quote paths in "$(cat "$W/brief.md")" - `[R] Rust` has a space. sed substitutions in launchers
  use `|` delimiters (paths contain `/`).
- Removing a trailing comment line leaves a blank line: run rustfmt on edited files.
- Wi-Fi (PosteMobile router) dropped several times in the evening (beacon loss / ssid-not-found):
  router side, not the tests. A network test failing at such a moment is a flake, re-run once.
