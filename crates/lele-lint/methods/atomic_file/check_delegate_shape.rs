use crate::common;
use crate::rules;
use crate::Identifier;
use crate::Project;
use crate::RuleViolation;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::path::Path;

pub fn check_delegate_shape(
    rel_path: &Path,
    file: &syn::File,
    project: &Project,
    known_stems: &HashSet<String>,
    declared: &BTreeMap<String, common::DeclaredType>,
) -> Vec<RuleViolation> {
    let Some(file_stem) = rel_path.file_stem().and_then(|s| s.to_str()) else {
        return Vec::new();
    };
    if common::SourceFile(file)
        .primary_type_name(file_stem)
        .is_some()
    {
        return Vec::new();
    }
    let Some(type_snake) = Identifier(file_stem).longest_type_prefix(known_stems) else {
        return Vec::new();
    };
    let Some(rest) = file_stem
        .strip_prefix(type_snake)
        .and_then(|tail| tail.strip_prefix('_'))
    else {
        return Vec::new();
    };
    if rest.is_empty() {
        return Vec::new();
    }
    let type_name = Identifier(type_snake).to_pascal_case();
    let is_delegate = declared
        .get(type_snake)
        .is_some_and(|entry| entry.methods.contains(rest));
    let message = if is_delegate {
        format!(
            "delegate method file `{file_stem}.rs` for `{type_name}::{rest}` must live at `methods/{type_snake}/{rest}.rs`, not in `src/`"
        )
    } else {
        format!(
            "filename mismatch — `{file_stem}.rs` looks like a method file for `{type_name}` but `{rest}` is not an atomic delegate; rename to `{rest}.rs`"
        )
    };
    vec![RuleViolation::from_file(
        project.src_dir.join(rel_path),
        rules::atomic_file::AtomicFile::CODE,
        message,
    )]
}

// no test_usage necessary
