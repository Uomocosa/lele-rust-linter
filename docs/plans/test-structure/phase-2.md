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

## Fleet
- Delete forbidden test modules (and their helpers). Before deleting a `__bevy_systems__` test,
  move it into the owning `*_plugin.rs` tests ONLY if it asserts plugin-visible behaviour not
  covered elsewhere. Few moves expected. Moved tests build the app with the plugin, not the system.
- Assertions on constants/tuning values are deleted (D8).
- p2p: `crates/lobby-room/bevy_plugin/src/runner_config_tests.rs` -> colocate;
  `crates/freenet-libp2p-bevy-core/tests/spectator_fake_network.rs` -> colocate in the owning
  plugin file or list core as not e2e and move it. Ask the orchestrator if unclear.
- Report new test count and `just test` time for p2p.
