# HANDOFF: test-structure refactor (state at end of 2026-11-19)

Read this first, then PLAN.md (decisions D1-D15, rule table, log) and phase-3.md / phase-4.md.
Run as orchestrator with `/opencode-orchestrator` + the `lele-linter-change-rs` flow.

## Where we are
- Branch `feature/test-structure` in all 7 fleet repos; every tree clean; all pushed.
- Phase 0 (nextest tooling), Phase 1 (rename LELE006/007/009 -> LELETEST001-003) and Phase 2
  (LELETEST002 ext, LELETEST004, LELEBEVYTEST001, D12 preview exemption) are DONE fleet-wide.
  Every lint target of every repo is clean.
- 2026-11-19: all phase-2 open items decided and done (PLAN.md log): telegram-bot-helper 1c64193
  (lele-bevy-preview `scene::Clock`, `Scene::hover::<C>`), p2p 3a746c2 (previews restored on
  Clock/hover) + e31f055 (`Room::rendezvous_preview_dial`). D5 removed (no usage-count rule),
  D10 = guidance only; phase 3/4 codes renumbered (phase 3: LELETEST005-006, phase 4: LELETEST007).
- `~/.config/opencode` (skills) is pushed (05048a7). The user's own uncommitted changes
  (orchestrator-core, ui-sketch-rs) are still there: never touch those.
- p2p numbers: 1554 tests before -> 1208 after (all green, 31 skipped). Suite ~269 s test time,
  dominated by mp4/scene previews and multi-swarm plugin tests.

## Phase 3: DONE (2026-11-19)
LELETEST005/006 in lele-lint a94e93e. Fleet: harness 9cd82f3, release 26e927c, state-machine 8dd8395,
telegram 4390c38, p2p 9bc8b5a (1396 tests / 246 s). Taxonomy had 0 hits. See PLAN.md log.
Phase-3 tooling (p2p .local-run): p3-p2p-plan.md, p3-decisions.md, p3-fleet-brief.md, launch3.sh JOB,
check3.sh JOB; linter .local-run: p3-small-brief.md, launch-small3.sh, read-job.sh.

## Phase 4: DONE (2026-11-19)
LELETEST007 in lele-lint 0ba628b. Fleet: harness 20d488c, taxonomy 8935a11, telegram b5e8863,
release f300acd, p2p 440ff21 (1398 tests / 257 s), state-machine had 0 hits. Skill:
~/.config/opencode 4809fa3 (lele-syntax-rs RATIONALE §19). PLAN DONE; branches NOT merged.

## Open decisions for the USER (ask before acting)
Items 1-5 of 2026-11-18 decided on 2026-11-19 (PLAN.md D5 revised, D13-D15, log). Remaining:
None (D10 decided 2026-11-19: guidance only, no lint).

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
