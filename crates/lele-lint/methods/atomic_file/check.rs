use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check(_self: &rules::atomic_file::AtomicFile, project: &Project) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    let known_stems = rules::atomic_file::AtomicFile::known_type_stems(project);
    let declared = project.collect_declared();

    for (rel_path, file) in &project.parsed_files {
        if project.is_exempt_source_path(rel_path) {
            continue;
        }
        violations.extend(rules::atomic_file::AtomicFile::check_delegate_shape(
            rel_path,
            file,
            project,
            &known_stems,
            &declared,
        ));
        violations.extend(rules::atomic_file::AtomicFile::check_atomic_file(
            file,
            rel_path,
            project,
            &known_stems,
        ));
    }

    violations
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::DunderPath;
    use crate::Identifier;
    use std::collections::HashSet;

    use super::check;
    use crate::rules::atomic_file::AtomicFile;
    use crate::Project;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_usage() {
        let file: syn::File =
            syn::parse_str("pub struct AtomicFile;\npub fn helper() {}\nfn private() {}").unwrap();
        let items = rules::atomic_file::AtomicFile::collect_pub_items(&file);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].name, "AtomicFile");
        assert_eq!(items[1].name, "helper");

        let project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        assert!(project.is_exempt_source_path(Path::new("rules/mod.rs")));
        assert!(!project.is_exempt_source_path(Path::new("rules/atomic_file.rs")));
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

        let violations = rules::atomic_file::AtomicFile::check_filename_match(
            "load",
            "lele_config_load",
            Path::new("lele_config_load.rs"),
            &project,
            &known,
        );
        assert!(violations.is_empty());

        let violations = rules::atomic_file::AtomicFile::check_filename_match(
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
        let violations = rules::atomic_file::AtomicFile::check_fn_file_purity(
            &file,
            "discover",
            Path::new("discover.rs"),
            &project,
        );
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("must hold only its fn"));
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
        let violations = rules::atomic_file::AtomicFile::check_fn_file_purity(
            &file,
            "discover",
            Path::new("discover.rs"),
            &project,
        );
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
        let violations = rules::atomic_file::AtomicFile::check_fn_file_purity(
            &file,
            "discover",
            Path::new("discover.rs"),
            &project,
        );
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
                .any(|d| d.message.contains("rename it to `parse_source_files.rs`")),
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
                .contains("move it to `methods/project/discover.rs`")),
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
