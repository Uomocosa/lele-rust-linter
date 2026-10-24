use crate::rules;
use crate::Project;
use crate::RuleViolation;
use std::collections::HashSet;
use std::path::Path;

pub fn check_atomic_file(
    file: &syn::File,
    rel_path: &Path,
    project: &Project,
    known_stems: &HashSet<String>,
) -> Vec<RuleViolation> {
    let pub_items = rules::atomic_file::AtomicFile::collect_pub_items(file);
    let Some(primary) = pub_items.first() else {
        return Vec::new();
    };
    let file_stem = rel_path.file_stem().and_then(|s| s.to_str()).unwrap_or("");

    let mut violations = rules::atomic_file::AtomicFile::check_filename_match(
        &primary.name,
        file_stem,
        rel_path,
        project,
        known_stems,
    );
    for extra in pub_items.get(1..).unwrap_or(&[]) {
        let suggested_file = format!("{}_{}.rs", file_stem, extra.name);
        violations.push(RuleViolation::from_file(
            project.src_dir.join(rel_path),
            rules::atomic_file::AtomicFile::CODE,
            format!(
                "only one public item per file — move `pub {} {}` to `{}`",
                extra.kind.kind_str(),
                extra.name,
                suggested_file
            ),
        ));
    }
    violations.extend(rules::atomic_file::AtomicFile::check_fn_file_purity(
        file, file_stem, rel_path, project,
    ));
    violations
}

// no test_usage necessary
