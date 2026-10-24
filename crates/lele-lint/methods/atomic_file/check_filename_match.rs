use crate::rules;
use crate::Identifier;
use crate::Project;
use crate::RuleViolation;
use lele_snake_case::ToLeleSnakeCase;
use std::collections::HashSet;
use std::path::Path;

pub fn check_filename_match(
    name: &str,
    file_stem: &str,
    rel_path: &Path,
    project: &Project,
    known_stems: &HashSet<String>,
) -> Vec<RuleViolation> {
    let expected = name.to_snake_case();

    if expected == file_stem {
        return Vec::new();
    }

    if file_stem.contains('_')
        && Identifier(file_stem)
            .longest_type_prefix(known_stems)
            .is_some()
    {
        if let Some((_prefix, suffix)) = file_stem.rsplit_once('_') {
            if expected.ends_with(suffix) {
                return Vec::new();
            }
        }
    }

    vec![RuleViolation::from_file(
        project.src_dir.join(rel_path),
        rules::atomic_file::AtomicFile::CODE,
        format!("filename mismatch — `{file_stem}.rs` should be `{expected}.rs`"),
    )]
}

// no test_usage necessary
