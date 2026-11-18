# Phase 2: forbidden tests

Master plan: PLAN.md (D3, D4, D8, standing authorisations). Branch feature/test-structure.

## Linter
- LELEBEVYTEST001 (lele-bevy-lint): any `#[cfg(test)]` in a file under a `__bevy_systems__/`
  (or `bevy_systems/`, the same role) folder is an error.
- LELETEST004 (lele-lint): no `#[cfg(test)]` in a plain-data / trivial-constructor file.
  Heuristic (linter worker writes the final version here before coding): the file's primary
  item is a struct/enum whose only inherent/trait methods are `new`/`default`/`From::from`
  (or none), and those bodies only build the value from args/literals/constants.
- LELETEST003: files flagged by LELETEST004 or LELEBEVYTEST001, and `__basic__/` files, need no
  `test_usage` and no opt-out comment; an opt-out comment there is itself an error.
- LELETEST002: a file named `*_tests.rs` under src/ is an error; a crate-level `tests/` dir is an
  error unless the crate is listed as e2e (`[lele.config] e2e_crates = [..]`, new key, paths
  relative to the config).

## Orchestrator decisions (follow from the plan)
- LELETEST004 does NOT reuse `is_exempt` (also exempts dunder paths, main.rs, tests/ dirs) nor
  `is_type_only` (it ignores ALL trait impls, so `struct XPlugin` + `impl Plugin` counts as
  type-only and plugin tests would be banned). Own predicate `is_plain_data(file)`:
  the file has a struct/enum; every `impl` block (inherent or trait) holds only fns named
  `new`, `default` or `from` (plus assoc consts); each such fn body has no `if`, `match`,
  loop, `?`, closure, or `.await`. Forbidden = `is_plain_data` ||
  (`mod.rs`/`lib.rs` && `is_pure_module_tree`) || `constants.rs`. Never forbidden: any file
  with an `impl Plugin`, `*_plugin.rs`, `is_atomic_delegate_only` files (phase 4 wants their
  `test_usage`, D4).
- lele-bevy-lint `tests/rule_examples.rs` contiguity test becomes per family like phase 1
  (`LELEBEVY001..N`, `LELEBEVYTEST001..N`).
- Linter work is three commits/jobs: A = LELEBEVYTEST001; B = LELETEST004 + LELETEST003 change;
  C = LELETEST002 extension + `e2e_crates` config key.

## Fleet
- Delete forbidden test modules (and their helpers). Before deleting a `__bevy_systems__` test,
  move it into the owning `*_plugin.rs` tests ONLY if it asserts plugin-visible behaviour not
  covered elsewhere. Few moves expected. Moved tests build the app with the plugin, not the system.
- Assertions on constants/tuning values are deleted (D8).
- p2p: `crates/lobby-room/bevy_plugin/src/runner_config_tests.rs` -> colocate;
  `crates/freenet-libp2p-bevy-core/tests/spectator_fake_network.rs` -> colocate in the owning
  plugin file or list core as not e2e and move it. Ask the orchestrator if unclear.
- Report new test count and `just test` time for p2p.
