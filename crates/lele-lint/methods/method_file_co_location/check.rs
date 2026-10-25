use std::collections::HashSet;
use std::path::Path;

use crate::common;
use crate::rules;
use crate::Identifier;
use crate::Project;
use crate::RuleViolation;

pub fn check(
    _self: &rules::method_file_co_location::MethodFileCoLocation,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    let type_dirs = rules::method_file_co_location::MethodFileCoLocation::build_type_map(project);
    let known_stems: HashSet<String> = type_dirs.keys().cloned().collect();

    for (rel_path, file) in &project.parsed_files {
        let Some(file_stem) = rel_path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };

        if common::SourceFile(file)
            .primary_type_name(file_stem)
            .is_some()
        {
            continue;
        }

        let file_dir = rel_path.parent().unwrap_or(Path::new(""));

        if let Some(candidate_dirs) =
            rules::method_file_co_location::MethodFileCoLocation::find_parent_type(
                file_stem,
                &known_stems,
                &type_dirs,
            )
        {
            if !candidate_dirs.iter().any(|dir| dir == file_dir) {
                let Some(type_snake) = Identifier(file_stem).longest_type_prefix(&known_stems)
                else {
                    continue;
                };
                let candidates = candidate_dirs
                    .iter()
                    .map(|dir| dir.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                violations.push(RuleViolation::from_file(project.src_dir.join(rel_path), rules::method_file_co_location::MethodFileCoLocation::CODE, format!(
                        "method file `{}` is not next to `{type_snake}.rs` — move it into the folder holding `{type_snake}.rs` (found in: {candidates})",
                        rel_path.display()
                    )));
            }
        } else if let Some(suffix) = file_stem.rsplit('_').next() {
            if file_stem.contains('_')
                && rules::method_file_co_location::MethodFileCoLocation::pub_fn_names(file)
                    .iter()
                    .any(|n| n == suffix)
            {
                violations.push(RuleViolation::from_file(project.src_dir.join(rel_path), rules::method_file_co_location::MethodFileCoLocation::CODE, format!(
                        "orphan method file `{file_stem}.rs` — no parent type found; rename to `{suffix}.rs` or `<type>_{suffix}.rs` with a `<type>.rs` defining the type",
                    )));
            }
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::Identifier;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::path::PathBuf;

    use super::check;
    use crate::rules::method_file_co_location::MethodFileCoLocation;
    use crate::Project;

    fn known_stems(dirs: &HashMap<String, Vec<PathBuf>>) -> HashSet<String> {
        dirs.keys().cloned().collect()
    }

    #[test]
    fn test_usage() {
        let mut dirs = HashMap::new();
        dirs.insert("freenet_client".to_string(), vec![PathBuf::from("freenet")]);
        dirs.insert("config".to_string(), vec![PathBuf::from("clicker")]);

        assert_eq!(
            rules::method_file_co_location::MethodFileCoLocation::find_parent_type(
                "freenet_client_connect",
                &known_stems(&dirs),
                &dirs
            ),
            Some(vec![PathBuf::from("freenet")])
        );
        assert_eq!(
            rules::method_file_co_location::MethodFileCoLocation::find_parent_type(
                "config_new",
                &known_stems(&dirs),
                &dirs
            ),
            Some(vec![PathBuf::from("clicker")])
        );
        assert_eq!(
            rules::method_file_co_location::MethodFileCoLocation::find_parent_type(
                "bevy_systems",
                &known_stems(&dirs),
                &dirs
            ),
            None
        );
    }

    #[test]
    fn test_usage_longest_prefix_wins() {
        let mut dirs = HashMap::new();
        dirs.insert("freenet".to_string(), vec![PathBuf::from("freenet")]);
        dirs.insert("freenet_client".to_string(), vec![PathBuf::from("freenet")]);

        assert_eq!(
            Identifier("freenet_client_connect").longest_type_prefix(&known_stems(&dirs)),
            Some("freenet_client")
        );
    }

    #[test]
    fn test_usage_duplicate_type_dirs() {
        let mut dirs = HashMap::new();
        dirs.insert(
            "plugin".to_string(),
            vec![PathBuf::from("boxes"), PathBuf::from("roster")],
        );

        assert_eq!(
            rules::method_file_co_location::MethodFileCoLocation::find_parent_type(
                "plugin_build",
                &known_stems(&dirs),
                &dirs
            ),
            Some(vec![PathBuf::from("boxes"), PathBuf::from("roster")])
        );
    }

    #[test]
    fn test_usage_pub_fn_names() {
        let file: syn::File =
            syn::parse_str("pub fn load() {}\nfn private() {}\npub struct Helper;").unwrap();
        assert_eq!(
            rules::method_file_co_location::MethodFileCoLocation::pub_fn_names(&file),
            vec!["load".to_string()]
        );
    }

    fn orphan_project(files: &[(&str, &str)]) -> Project {
        let mut parsed_files = std::collections::HashMap::new();
        for (name, code) in files {
            parsed_files.insert(PathBuf::from(name), syn::parse_str(code).unwrap());
        }
        Project {
            root: PathBuf::from("."),
            src_dir: PathBuf::from("src"),
            entries: Vec::new(),
            module_info: std::collections::HashMap::default(),
            parsed_files,
            ..Project::default()
        }
    }

    #[test]
    fn test_usage_orphan_method_file_flagged() {
        let project = orphan_project(&[
            ("lele_config.rs", "pub struct LeleConfig { pub x: u8 }"),
            ("config_load.rs", "pub fn load() {}"),
        ]);
        let violations = check(&MethodFileCoLocation, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("orphan method file"));
    }

    #[test]
    fn test_usage_plain_free_function_passes() {
        let project = orphan_project(&[
            ("lele_config.rs", "pub struct LeleConfig { pub x: u8 }"),
            ("load.rs", "pub fn load() {}"),
        ]);
        let violations = check(&MethodFileCoLocation, &project);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_exact_match_multi_underscore_passes() {
        let project = orphan_project(&[(
            "to_snake_case.rs",
            "pub fn to_snake_case(input: &str) -> String { input.to_string() }",
        )]);
        let violations = check(&MethodFileCoLocation, &project);
        assert!(violations.is_empty());
    }
}
