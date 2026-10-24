use std::collections::BTreeSet;
use std::path::PathBuf;

use lele_bevy_lint::BevyLinter;
use lele_lint::Project;
use rstest::rstest;
use serde::Deserialize;

#[derive(Deserialize)]
struct Expected {
    violations: Vec<ExpectedViolation>,
}

#[derive(Deserialize)]
struct ExpectedViolation {
    file: String,
    code: String,
}

fn test_crate_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../lele-lint-test-crates/lele-bevy-lint")
        .join(name)
}

fn findings(name: &str) -> Result<BTreeSet<(String, String)>, Box<dyn std::error::Error>> {
    let root = test_crate_path(name);
    let project = Project::try_from_start_dir_and_scan_folders(Some(&root), None)?;
    Ok(BevyLinter::default()
        .rules
        .iter()
        .flat_map(|rule| rule.check(&project))
        .map(|violation| {
            let file = violation
                .file
                .strip_prefix(&root)
                .unwrap_or(&violation.file);
            (file.display().to_string(), violation.code.to_string())
        })
        .collect())
}

fn expected(name: &str) -> Result<BTreeSet<(String, String)>, Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string(test_crate_path(name).join("expected.toml"))?;
    let parsed: Expected = toml::from_str(&text)?;
    Ok(parsed
        .violations
        .into_iter()
        .map(|violation| (violation.file, violation.code))
        .collect())
}

/// Every test crate reports exactly the `(file, code)` pairs in its `expected.toml`.
#[rstest]
#[case("compliant_crate")]
#[case("violation_crate")]
fn test_crate_matches_expected(#[case] name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let found = findings(name)?;
    let wanted = expected(name)?;
    let missing: Vec<_> = wanted.difference(&found).collect();
    let unexpected: Vec<_> = found.difference(&wanted).collect();
    if missing.is_empty() && unexpected.is_empty() {
        return Ok(());
    }
    Err(format!("{name}:\nmissing: {missing:#?}\nunexpected: {unexpected:#?}").into())
}
