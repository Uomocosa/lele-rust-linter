use std::collections::BTreeSet;
use std::path::PathBuf;

use lele_bevy_lint::checkers::build_checkers;
use lele_lint::Project;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_fixtures")
        .join(name)
}

fn findings(name: &str) -> BTreeSet<(String, String)> {
    let root = fixture_path(name);
    let discovered = Project::discover(Some(&root), None);
    assert!(discovered.is_ok(), "fixture discover failed for {name}");
    let Ok(project) = discovered else {
        return BTreeSet::new();
    };
    build_checkers()
        .iter()
        .flat_map(|checker| checker.check(&project))
        .map(|violation| {
            let file = violation
                .file
                .strip_prefix(&root)
                .unwrap_or(&violation.file)
                .display()
                .to_string();
            (file, violation.code.to_string())
        })
        .collect()
}

fn expected(pairs: &[(&str, &str)]) -> BTreeSet<(String, String)> {
    pairs
        .iter()
        .map(|(file, code)| ((*file).to_string(), (*code).to_string()))
        .collect()
}

#[test]
fn compliant_crate_has_no_violations() {
    let found = findings("compliant_crate");
    assert!(
        found.is_empty(),
        "expected no violations in compliant crate, got {found:?}"
    );
}

#[test]
fn violation_crate_reports_exactly_the_expected_findings() {
    let code_of = |name: &str| {
        build_checkers()
            .iter()
            .find(|checker| checker.name() == name)
            .map(|checker| checker.code())
            .expect("unknown checker")
    };
    let found = findings("violation_crate");
    let wanted = expected(&[
        ("src/bad_bevy_export/mod.rs", code_of("bevy_export")),
        (
            "src/bad_bevy_folder/tick_enemies.rs",
            code_of("bevy_folder"),
        ),
        ("src/bad_bevy_plugin/plugin.rs", code_of("bevy_ui")),
        (
            "src/bad_bevy_plugin/plugin.rs",
            code_of("bevy_plugin_scene"),
        ),
        (
            "src/bad_bevy_plugin_spread/bevy_systems/spawn_hud.rs",
            code_of("bevy_ui"),
        ),
        (
            "src/bad_bevy_plugin_spread/plugin.rs",
            code_of("bevy_plugin_scene"),
        ),
        ("src/bad_bevy_ui/chained.rs", code_of("bevy_ui")),
        ("src/bad_bevy_ui/copy_preview.rs", code_of("bevy_ui")),
        ("src/bad_bevy_ui/cursor.rs", code_of("bevy_ui")),
        ("src/bad_bevy_ui/discarded.rs", code_of("preview_routing")),
        ("src/bad_bevy_ui/missing_mp4.rs", code_of("bevy_ui_mp4")),
        ("src/bad_bevy_ui/no_contract.rs", code_of("bevy_ui")),
        ("src/bad_bevy_ui/no_contract.rs", code_of("preview_routing")),
        ("src/bad_bevy_ui/restyle.rs", code_of("bevy_ui_restyle")),
    ]);
    let missing: Vec<_> = wanted.difference(&found).collect();
    let unexpected: Vec<_> = found.difference(&wanted).collect();
    assert!(
        missing.is_empty() && unexpected.is_empty(),
        "missing findings: {missing:?}\nunexpected findings: {unexpected:?}"
    );
}
