use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use lele_bevy_lint::BevyLinter;
use lele_lint::ExampleFile;
use lele_lint::Location;
use lele_lint::Project;
use lele_lint::Rule;

const DEFAULT_CARGO_TOML: &str = r#"[package]
name = "example"
version = "0.1.0"
edition = "2021"

[workspace]
"#;

const DEFAULT_LELE_TOML: &str = "[lele.lint]\n";

fn write_file(root: &Path, rel: &str, source: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = root.join(rel);
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::write(path, source)?;
    Ok(())
}

fn materialize(root: &Path, files: &[ExampleFile]) -> Result<(), Box<dyn std::error::Error>> {
    write_file(root, "Cargo.toml", DEFAULT_CARGO_TOML)?;
    write_file(root, "lele.toml", DEFAULT_LELE_TOML)?;
    write_file(root, "src/lib.rs", "")?;
    for file in files {
        write_file(root, file.path, file.source)?;
    }
    Ok(())
}

fn lint(files: &[ExampleFile]) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    materialize(dir.path(), files)?;
    let mut project = Project::try_from_start_dir_and_scan_folders(Some(dir.path()), None)?;
    let config = project.load_config().unwrap_or_default();
    project.apply_layout(&config)?;
    let root: PathBuf = dir.path().to_path_buf();
    let mut lines: Vec<String> = BevyLinter::default()
        .rules
        .iter()
        .flat_map(|rule| rule.check(&project))
        .map(|violation| {
            let file = violation
                .file
                .strip_prefix(&root)
                .unwrap_or(&violation.file);
            let line = match violation.location {
                Location::File => 1,
                Location::Site { line, .. } => line,
            };
            format!(
                "{} {}:{} {}",
                violation.code,
                file.display(),
                line,
                violation.message
            )
        })
        .collect();
    lines.sort();
    Ok(lines)
}

fn check_rule(rule: &dyn Rule) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut problems = Vec::new();
    let code = rule.code();
    let doc = rule.doc();
    if doc.summary.is_empty() || doc.why.is_empty() {
        problems.push(format!("{code}: summary and why must not be empty"));
    }
    if doc.bad.is_empty() || doc.good.is_empty() {
        problems.push(format!("{code}: needs at least one bad and one good file"));
        return Ok(problems);
    }
    let bad = lint(doc.bad)?;
    if !bad.iter().any(|line| line.starts_with(code)) {
        problems.push(format!(
            "{code}: bad example does not trigger {code}; got:\n  {}",
            bad.join("\n  ")
        ));
    }
    let good = lint(doc.good)?;
    if !good.is_empty() {
        problems.push(format!(
            "{code}: good example must be clean; got:\n  {}",
            good.join("\n  ")
        ));
    }
    Ok(problems)
}

/// Every rule's bad example triggers its own code and its good example lints clean.
#[test]
fn every_rule_example_is_correct() -> Result<(), Box<dyn std::error::Error>> {
    let mut problems = Vec::new();
    let linter = BevyLinter::default();
    for rule in &linter.rules {
        problems.extend(check_rule(rule.as_ref())?);
    }
    if problems.is_empty() {
        return Ok(());
    }
    Err(problems.join("\n").into())
}

/// Every rule file is registered in `BevyLinter`, and codes are unique.
#[test]
fn every_rule_file_is_registered() -> Result<(), Box<dyn std::error::Error>> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/rules");
    let mut files = BTreeSet::new();
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        if stem != "mod" {
            files.insert(stem.to_string());
        }
    }
    let linter = BevyLinter::default();
    let rules = &linter.rules;
    let names: BTreeSet<String> = rules.iter().map(|c| c.name().to_string()).collect();
    let codes: BTreeSet<&str> = rules.iter().map(|c| c.code()).collect();
    if files != names {
        return Err(format!("rule files {files:?} != registered {names:?}").into());
    }
    if codes.len() != rules.len() {
        return Err("duplicate rule codes".into());
    }
    Ok(())
}

/// Codes are exactly `LELEBEVY001..LELEBEVY<count>`, in registration order.
#[test]
fn codes_are_contiguous_in_registration_order() {
    for (index, rule) in BevyLinter::default().rules.iter().enumerate() {
        assert_eq!(
            rule.code(),
            format!("LELEBEVY{:03}", index + 1),
            "{}",
            rule.name()
        );
    }
}
