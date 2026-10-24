use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check(
    _self: &rules::container_placement::ContainerPlacement,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    let Some(folder) = project.dunder_dir() else {
        return violations;
    };

    for source in project.content_sources() {
        let rel_path = source.relative_path;
        let file = source.file;
        let file_path = project.absolute_path(source.origin, rel_path);
        if project.is_in_dunder_dir(rel_path) {
            violations.extend(
                rules::container_placement::ContainerPlacement::check_container_file(
                    rel_path, file, project, &file_path,
                ),
            );
        } else {
            violations.extend(
                rules::container_placement::ContainerPlacement::check_free_file(
                    rel_path, file, &folder, &file_path,
                ),
            );
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::DunderPath;
    use std::path::PathBuf;

    use super::check;
    use crate::rules::container_placement::ContainerPlacement;
    use crate::Project;

    fn project(files: &[(&str, &str)]) -> Project {
        let mut project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        for (path, source) in files {
            let file: syn::File = syn::parse_str(source).unwrap();
            project.parsed_files.insert(PathBuf::from(path), file);
        }
        project
    }

    #[test]
    fn test_usage() {
        let project = project(&[(
            "discovery/game.rs",
            "#[derive(Debug)] pub struct Game(pub String);\n",
        )]);
        let violations = check(&ContainerPlacement, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("newtypes.rs"));
        assert!(violations[0].message.contains("discovery/__basic__"));
    }

    #[test]
    fn test_usage_allows_container_item() {
        let project = project(&[(
            "discovery/__basic__/newtypes.rs",
            "#[derive(Debug, Clone)] pub struct Game(pub String);\n",
        )]);
        let violations = check(&ContainerPlacement, &project);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_flags_behavior_in_container() {
        let project = project(&[(
            "discovery/__basic__/newtypes.rs",
            "#[derive(Debug)] pub struct Game(pub String);\nimpl Game { pub fn name(&self) -> &str { \"\" } }\n",
        )]);
        let violations = check(&ContainerPlacement, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("has impls"));
    }

    #[test]
    fn test_usage_container_dir() {
        assert_eq!(
            rules::container_placement::ContainerPlacement::container_dir(
                &PathBuf::from("discovery/session/room.rs"),
                "__basic__"
            ),
            PathBuf::from("discovery/__basic__")
        );
        assert_eq!(
            rules::container_placement::ContainerPlacement::container_dir(
                &PathBuf::from("lib_types.rs"),
                "__basic__"
            ),
            PathBuf::from("__basic__")
        );
    }
}
