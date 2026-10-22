use std::path::PathBuf;

use lele_lint::config::Config;
use lele_lint::Linter;
use lele_lint::Project;
use lele_lint::RuleViolation;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_fixtures")
        .join(name)
}

fn run_checkers(path: &str) -> Result<Vec<RuleViolation>, Box<dyn std::error::Error>> {
    let mut p = Project::discover(Some(&fixture_path(path)), None)?;
    let config = Config::load(&p.root).unwrap_or_default();
    p.apply_layout(&config)?;
    let checkers = Linter::default().checkers;
    Ok(checkers.iter().flat_map(|c| c.check(&p)).collect())
}

#[test]
fn compliant_crate_has_no_violations() -> Result<(), Box<dyn std::error::Error>> {
    let violations = run_checkers("compliant_crate")?;
    if !violations.is_empty() {
        return Err(
            format!("expected no violations in compliant crate, got {violations:?}").into(),
        );
    }
    Ok(())
}

#[test]
fn violation_crate_catches_all_errors() -> Result<(), Box<dyn std::error::Error>> {
    let violations = run_checkers("violation_crate")?;
    let codes: Vec<&str> = violations.iter().map(|d| d.code).collect();
    let code_of = |name: &str| {
        Linter::default()
            .checkers
            .iter()
            .find(|checker| checker.name() == name)
            .map(|checker| checker.code())
            .expect("unknown checker")
    };

    let expected = [
        code_of("atomic_file"),
        code_of("snake_case_files"),
        code_of("method_visibility"),
        code_of("no_cross_domain_reexport"),
        code_of("test_usage"),
        code_of("test_inline"),
        code_of("no_positional"),
        code_of("no_trivial_accessors"),
        code_of("domain_import"),
        code_of("atomic_delegates"),
        code_of("constructor_no_skip"),
        code_of("single_caller_type"),
        code_of("method_file_co_location"),
        code_of("single_field_newtype"),
        code_of("mod_rs_purity"),
        code_of("no_crate_paths"),
        code_of("root_reexport"),
        code_of("no_stuttered_path"),
        code_of("no_stuttered_type"),
    ];

    for code in expected {
        if !codes.contains(&code) {
            return Err(format!("expected {code} in violation crate, got codes: {codes:?}").into());
        }
    }
    Ok(())
}

#[test]
fn methods_crate_has_no_violations() -> Result<(), Box<dyn std::error::Error>> {
    let violations = run_checkers("methods_crate")?;
    if !violations.is_empty() {
        return Err(format!("expected no violations in methods crate, got {violations:?}").into());
    }
    Ok(())
}

#[test]
fn methods_violation_crate_catches_layout_errors() -> Result<(), Box<dyn std::error::Error>> {
    let violations = run_checkers("methods_violation_crate")?;
    let codes: Vec<&str> = violations.iter().map(|d| d.code).collect();
    let code_of = |name: &str| {
        Linter::default()
            .checkers
            .iter()
            .find(|checker| checker.name() == name)
            .map(|checker| checker.code())
            .expect("unknown checker")
    };

    for code in [
        code_of("methods_layout"),
        code_of("no_comments"),
        code_of("delegate_macro"),
    ] {
        if !codes.contains(&code) {
            return Err(format!(
                "expected {code} in methods violation crate, got codes: {codes:?}"
            )
            .into());
        }
    }
    Ok(())
}
