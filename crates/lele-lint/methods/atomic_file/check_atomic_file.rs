use crate::rules;
use crate::Project;
use crate::RuleViolation;
use lele_snake_case::ToLeleSnakeCase;
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
        let kind = extra.kind.kind_str();
        let message = if kind == "fn" {
            format!(
                "only one public item per file — make `pub fn {}` a method of the type it works on (first argument or return type), with its body in `methods/<type>/{}.rs`",
                extra.name, extra.name
            )
        } else {
            format!(
                "only one public item per file — move `pub {kind} {}` to its own `{}.rs`",
                extra.name,
                extra.name.to_snake_case()
            )
        };
        violations.push(RuleViolation::from_file(
            project.src_dir.join(rel_path),
            rules::atomic_file::AtomicFile::CODE,
            message,
        ));
    }
    violations.extend(rules::atomic_file::AtomicFile::check_fn_file_purity(
        file, file_stem, rel_path, project,
    ));
    violations
}

// no test_usage necessary
