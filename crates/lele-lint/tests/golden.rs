use std::path::Path;
use std::path::PathBuf;

use lele_lint::Linter;
use lele_lint::Location;
use lele_lint::Project;

const FIXTURES: [&str; 4] = [
    "compliant_crate",
    "violation_crate",
    "methods_crate",
    "methods_violation_crate",
];

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn render(fixture: &str) -> Result<String, Box<dyn std::error::Error>> {
    let root = manifest_dir().join("test_fixtures").join(fixture);
    let mut project = Project::try_from_start_dir_and_scan_folders(Some(&root), None)?;
    let config = project.load_config().unwrap_or_default();
    project.apply_layout(&config)?;
    let mut lines: Vec<String> = Linter::default()
        .checkers
        .iter()
        .flat_map(|c| c.check(&project))
        .map(|d| {
            let file = d.file.strip_prefix(&root).unwrap_or(&d.file);
            let (line, col) = match d.location {
                Location::File => (1, 0),
                Location::Site { line, col } => (line, col),
            };
            format!(
                "{}:{}:{} {} {}",
                file.display(),
                line,
                col,
                d.code,
                d.message
            )
        })
        .collect();
    lines.sort();
    let mut out = lines.join("\n");
    out.push('\n');
    Ok(out)
}

fn golden_path(fixture: &str) -> PathBuf {
    manifest_dir()
        .join("tests")
        .join("golden")
        .join(format!("{fixture}.txt"))
}

fn bless(path: &Path, actual: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::write(path, actual)?;
    Ok(())
}

/// Violations for every fixture must match `tests/golden/<fixture>.txt` exactly.
/// Run with `LELE_BLESS=1` to rewrite the golden files after an intended change.
#[test]
fn fixtures_match_golden_files() -> Result<(), Box<dyn std::error::Error>> {
    let blessing = std::env::var_os("LELE_BLESS").is_some();
    let mut mismatches = Vec::new();
    for fixture in FIXTURES {
        let actual = render(fixture)?;
        let path = golden_path(fixture);
        if blessing {
            bless(&path, &actual)?;
            continue;
        }
        let expected = std::fs::read_to_string(&path).unwrap_or_default();
        if expected != actual {
            mismatches.push(format!(
                "{fixture}: golden mismatch\n--- expected ({})\n{expected}\n--- actual\n{actual}",
                path.display()
            ));
        }
    }
    if mismatches.is_empty() {
        return Ok(());
    }
    Err(mismatches.join("\n").into())
}
