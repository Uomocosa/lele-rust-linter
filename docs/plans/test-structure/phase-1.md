# Phase 1: rename test rule codes (no behaviour change)

Master plan: docs/plans/test-structure/PLAN.md (decisions D1-D11). Branch feature/test-structure.

## Linter
- LELE006 -> LELETEST001, LELE007 -> LELETEST002, LELE009 -> LELETEST003. Same logic, same
  messages apart from the code. No aliases (D7).
- Update every reference in the linter repo: rule registry, rule_examples, tests, e2e
  fixtures, lele-lint-test-crates expectations, README, RULES.md / skills via `just docs`.
- Keep remaining LELE0xx numbers as they are (gaps are fine).
- Orchestrator decision (follows from the above): `tests/rule_examples.rs`
  `codes_are_contiguous_in_registration_order` becomes per family: `LELE` codes are
  LELE001..LELE043 minus an explicit retired list [006, 007, 009], in registration order;
  `LELETEST` codes are LELETEST001..N contiguous, in registration order. A retired number is
  never reused. Same idea for lele-bevy-lint when LELEBEVYTEST arrives (phase 2).

## Fleet
- Replace the old codes wherever they appear (lele.toml, docs, comments that are allowed).
- Skills in ~/.config/opencode/skills (lele-syntax-rs, lele-rs, bevy-rs, RATIONALE.md,
  SMELLS.md): replace old codes; separate small change, orchestrator's job.
- Expected new fleet errors: 0. Any non-zero count is a linter bug.
