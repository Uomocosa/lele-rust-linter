use lele_snake_case::ToLeleSnakeCase;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::path::Path;

use crate::checkers;
use crate::common;
use crate::Identifier;
use crate::Project;
use crate::RuleViolation;
use crate::Visibility;

pub fn check(_self: &checkers::atomic_file::AtomicFile, project: &Project) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    let known_stems = known_type_stems(project);
    let declared = project.collect_declared();

    for (rel_path, file) in &project.parsed_files {
        if project.is_exempt_source_path(rel_path) {
            continue;
        }
        violations.extend(check_delegate_shape(
            rel_path,
            file,
            project,
            &known_stems,
            &declared,
        ));
        violations.extend(check_atomic_file(file, rel_path, project, &known_stems));
    }

    violations
}

// needed helper: filename, single-public-item, and fn-file purity checks for one file
fn check_atomic_file(
    file: &syn::File,
    rel_path: &Path,
    project: &Project,
    known_stems: &HashSet<String>,
) -> Vec<RuleViolation> {
    let pub_items = collect_pub_items(file);
    let Some(primary) = pub_items.first() else {
        return Vec::new();
    };
    let file_stem = rel_path.file_stem().and_then(|s| s.to_str()).unwrap_or("");

    let mut violations =
        check_filename_match(&primary.name, file_stem, rel_path, project, known_stems);
    for extra in pub_items.get(1..).unwrap_or(&[]) {
        let suggested_file = format!("{}_{}.rs", file_stem, extra.name);
        violations.push(RuleViolation::from_file(
            project.src_dir.join(rel_path),
            checkers::atomic_file::AtomicFile::CODE,
            format!(
                "only one public item per file — move `pub {} {}` to `{}`",
                extra.kind.kind_str(),
                extra.name,
                suggested_file
            ),
        ));
    }
    violations.extend(check_fn_file_purity(file, file_stem, rel_path, project));
    violations
}

// needed helper: `<type>_<rest>.rs` must map to a declared delegate (or be renamed)
fn check_delegate_shape(
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
        checkers::atomic_file::AtomicFile::CODE,
        message,
    )]
}

// needed helper: SHAPE-F fn-file purity
fn check_fn_file_purity(
    file: &syn::File,
    file_stem: &str,
    rel_path: &Path,
    project: &Project,
) -> Vec<RuleViolation> {
    let has_pub_fn = file
        .items
        .iter()
        .any(|item| matches!(item, syn::Item::Fn(f) if Visibility(&f.vis).is_exposed()));
    if !has_pub_fn {
        return Vec::new();
    }
    let mut violations = Vec::new();
    for item in &file.items {
        let (name, kind) = match item {
            syn::Item::Struct(s) if Visibility(&s.vis).is_exposed() => {
                (s.ident.to_string(), "struct")
            }
            syn::Item::Enum(e) if Visibility(&e.vis).is_exposed() => (e.ident.to_string(), "enum"),
            syn::Item::Const(c) if Visibility(&c.vis).is_exposed() => {
                (c.ident.to_string(), "const")
            }
            syn::Item::Static(s) if Visibility(&s.vis).is_exposed() => {
                (s.ident.to_string(), "static")
            }
            syn::Item::Type(t) if Visibility(&t.vis).is_exposed() => (t.ident.to_string(), "type"),
            syn::Item::Trait(t) if Visibility(&t.vis).is_exposed() => {
                (t.ident.to_string(), "trait")
            }
            syn::Item::Union(u) if Visibility(&u.vis).is_exposed() => {
                (u.ident.to_string(), "union")
            }
            _ => continue,
        };
        let home = if kind == "const" || kind == "static" {
            "nearest `constants.rs`"
        } else {
            &format!("`{}.rs`", name.to_snake_case())
        };
        violations.push(RuleViolation::from_file(project.src_dir.join(rel_path), checkers::atomic_file::AtomicFile::CODE, format!(
                "SHAPE-F fn-file `{file_stem}.rs` must hold only the fn — move exposed `{kind} {name}` to {home} (O2-extraction)"
            )));
    }
    violations
}

struct PubItem {
    name: String,
    kind: PubItemKind,
}

enum PubItemKind {
    Struct,
    Enum,
    Fn,
}

impl PubItemKind {
    fn kind_str(&self) -> &'static str {
        match self {
            PubItemKind::Struct => "struct",
            PubItemKind::Enum => "enum",
            PubItemKind::Fn => "fn",
        }
    }
}

// needed helper: pub item collection
fn collect_pub_items(file: &syn::File) -> Vec<PubItem> {
    let mut items = Vec::new();
    for item in &file.items {
        match item {
            syn::Item::Struct(s) if matches!(s.vis, syn::Visibility::Public(_)) => {
                items.push(PubItem {
                    name: s.ident.to_string(),
                    kind: PubItemKind::Struct,
                });
            }
            syn::Item::Enum(e) if matches!(e.vis, syn::Visibility::Public(_)) => {
                items.push(PubItem {
                    name: e.ident.to_string(),
                    kind: PubItemKind::Enum,
                });
            }
            syn::Item::Fn(f) if matches!(f.vis, syn::Visibility::Public(_)) => {
                items.push(PubItem {
                    name: f.sig.ident.to_string(),
                    kind: PubItemKind::Fn,
                });
            }
            _ => {}
        }
    }
    items
}

// needed helper: filename validation against snake_case
fn check_filename_match(
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
        checkers::atomic_file::AtomicFile::CODE,
        format!("filename mismatch — `{file_stem}.rs` should be `{expected}.rs`"),
    )]
}

// needed helper: known primary-type file stems in the project
fn known_type_stems(project: &Project) -> HashSet<String> {
    project
        .parsed_files
        .iter()
        .filter_map(|(rel_path, file)| {
            let stem = rel_path.file_stem()?.to_str()?;
            common::SourceFile(file).primary_type_name(stem)?;
            Some(stem.to_string())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::DunderPath;
    use crate::Identifier;
    use std::collections::HashSet;

    use super::{check, check_filename_match, check_fn_file_purity, collect_pub_items};
    use crate::checkers::atomic_file::AtomicFile;
    use crate::Project;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_usage() {
        let file: syn::File =
            syn::parse_str("pub struct AtomicFile;\npub fn helper() {}\nfn private() {}").unwrap();
        let items = collect_pub_items(&file);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].name, "AtomicFile");
        assert_eq!(items[1].name, "helper");

        let project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        assert!(project.is_exempt_source_path(Path::new("checkers/mod.rs")));
        assert!(!project.is_exempt_source_path(Path::new("checkers/atomic_file.rs")));
    }

    #[test]
    fn test_usage_suffix_exemption_needs_known_parent() {
        let project = Project {
            root: std::path::PathBuf::from("."),
            src_dir: std::path::PathBuf::from("src"),
            entries: Vec::new(),
            module_info: std::collections::HashMap::default(),
            parsed_files: std::collections::HashMap::default(),
            ..Project::default()
        };
        let known: HashSet<String> = HashSet::from(["lele_config".to_string()]);
        let empty: HashSet<String> = HashSet::new();

        let violations = check_filename_match(
            "load",
            "lele_config_load",
            Path::new("lele_config_load.rs"),
            &project,
            &known,
        );
        assert!(violations.is_empty());

        let violations = check_filename_match(
            "load",
            "config_load",
            Path::new("config_load.rs"),
            &project,
            &empty,
        );
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("filename mismatch"));

        assert!(Identifier("freenet_client_connect")
            .longest_type_prefix(&known)
            .is_none());
        let with_prefix: HashSet<String> = HashSet::from(["freenet_client".to_string()]);
        assert_eq!(
            Identifier("freenet_client_connect").longest_type_prefix(&with_prefix),
            Some("freenet_client")
        );
    }

    #[test]
    fn test_usage_fn_file_purity_flags_exposed_type() {
        let project = Project {
            root: std::path::PathBuf::from("."),
            src_dir: std::path::PathBuf::from("src"),
            entries: Vec::new(),
            module_info: std::collections::HashMap::default(),
            parsed_files: std::collections::HashMap::default(),
            ..Project::default()
        };
        let file: syn::File =
            syn::parse_str("pub struct Cache { pub x: u32 }\npub fn discover() {}\n").unwrap();
        let violations =
            check_fn_file_purity(&file, "discover", Path::new("discover.rs"), &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("SHAPE-F"));
    }

    #[test]
    fn test_usage_fn_file_purity_allows_private_struct() {
        let project = Project {
            root: std::path::PathBuf::from("."),
            src_dir: std::path::PathBuf::from("src"),
            entries: Vec::new(),
            module_info: std::collections::HashMap::default(),
            parsed_files: std::collections::HashMap::default(),
            ..Project::default()
        };
        let file: syn::File =
            syn::parse_str("struct Cache { x: u32 }\npub fn discover() {}\n").unwrap();
        let violations =
            check_fn_file_purity(&file, "discover", Path::new("discover.rs"), &project);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_fn_file_purity_flags_pub_const() {
        let project = Project {
            root: std::path::PathBuf::from("."),
            src_dir: std::path::PathBuf::from("src"),
            entries: Vec::new(),
            module_info: std::collections::HashMap::default(),
            parsed_files: std::collections::HashMap::default(),
            ..Project::default()
        };
        let file: syn::File =
            syn::parse_str("pub const MAX: usize = 8;\npub fn discover() {}\n").unwrap();
        let violations =
            check_fn_file_purity(&file, "discover", Path::new("discover.rs"), &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("constants.rs"));
    }

    fn shape_project(files: &[(&str, &str)]) -> Project {
        let mut parsed_files = std::collections::HashMap::new();
        for (name, code) in files {
            parsed_files.insert(PathBuf::from(name), syn::parse_str(code).unwrap());
        }
        Project {
            root: PathBuf::from("."),
            src_dir: PathBuf::from("src"),
            parsed_files,
            ..Project::default()
        }
    }

    #[test]
    fn test_usage_flags_misnamed_free_fn_file() {
        let project = shape_project(&[
            (
                "project.rs",
                "pub struct Project;\n#[atomic_delegates]\nimpl Project { pub fn discover() {} }\n",
            ),
            (
                "project_parse_source_files.rs",
                "pub(crate) fn parse_source_files() {}\n",
            ),
        ]);
        let violations = check(&AtomicFile, &project);
        assert!(
            violations
                .iter()
                .any(|d| d.message.contains("rename to `parse_source_files.rs`")),
            "expected rename diagnostic, got {violations:?}"
        );
    }

    #[test]
    fn test_usage_flags_mislocated_delegate_file() {
        let project = shape_project(&[
            (
                "project.rs",
                "pub struct Project;\n#[atomic_delegates]\nimpl Project { pub fn discover() {} }\n",
            ),
            ("project_discover.rs", "pub fn discover() {}\n"),
        ]);
        let violations = check(&AtomicFile, &project);
        assert!(
            violations.iter().any(|d| d
                .message
                .contains("must live at `methods/project/discover.rs`")),
            "expected mislocated diagnostic, got {violations:?}"
        );
    }

    #[test]
    fn test_usage_type_file_with_underscore_stem_is_clean() {
        let project = shape_project(&[("player_id.rs", "pub struct PlayerId(pub u64);\n")]);
        let violations = check(&AtomicFile, &project);
        assert!(
            !violations
                .iter()
                .any(|d| d.message.contains("looks like a method file")),
            "unexpected method-file diagnostic, got {violations:?}"
        );
    }
}
