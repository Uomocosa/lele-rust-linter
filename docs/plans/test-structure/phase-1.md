# Phase 1: rename test rule codes (no behaviour change)

Master plan: docs/plans/test-structure/PLAN.md (decisions D1-D11). Branch feature/test-structure.

## Linter
- LELE006 -> LELETEST001, LELE007 -> LELETEST002, LELE009 -> LELETEST003. Same logic, same
  messages apart from the code. No aliases (D7).
- Update every reference in the linter repo: rule registry, rule_examples, tests, e2e
  fixtures, lele-lint-test-crates expectations, README, RULES.md / skills via `just docs`.
- Keep remaining LELE0xx numbers as they are (gaps are fine).

## Fleet
- Replace the old codes wherever they appear (lele.toml, docs, comments that are allowed).
- Skills in ~/.config/opencode/skills (lele-syntax-rs, lele-rs, bevy-rs, RATIONALE.md,
  SMELLS.md): replace old codes; separate small change, orchestrator's job.
- Expected new fleet errors: 0. Any non-zero count is a linter bug.
