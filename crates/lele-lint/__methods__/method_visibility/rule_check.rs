use crate::rules;
use crate::Identifier;
use crate::Project;
use crate::RuleViolation;

pub fn rule_check(
    _self: &rules::method_visibility::MethodVisibility,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    let dir_groups =
        rules::method_visibility::MethodVisibility::group_entries_by_parent_dir(&project.entries);

    for (parent_dir, files) in &dir_groups {
        let struct_names = rules::method_visibility::MethodVisibility::collect_struct_names(
            files, parent_dir, project,
        );

        for file_name in files.iter().filter(|f| f.ends_with(".rs")) {
            let Some(struct_name) = file_name
                .strip_suffix(".rs")
                .and_then(|stem| Identifier(stem).longest_type_prefix(&struct_names))
                .map(str::to_string)
            else {
                continue;
            };
            if !rules::method_visibility::MethodVisibility::is_actually_method_file(
                file_name, parent_dir, project,
            ) {
                continue;
            }

            let method_mod_name = file_name.strip_suffix(".rs").unwrap_or(file_name);

            if let Some(declared_pub) =
                rules::method_visibility::MethodVisibility::declared_as_pub_mod(
                    &project.module_info,
                    parent_dir,
                    method_mod_name,
                )
            {
                violations.push(RuleViolation::from_file(project.absolute_src_path(&declared_pub), rules::method_visibility::MethodVisibility::CODE, format!(
                        "method file `{}` of struct `{}` is declared `pub mod` — declare it with plain `mod`",
                        file_name, struct_name
                    )));
            }

            if let Some(reexported_at) =
                rules::method_visibility::MethodVisibility::reexported_in_pub_use(
                    &project.module_info,
                    parent_dir,
                    method_mod_name,
                )
            {
                violations.push(RuleViolation::from_file(
                    project.absolute_src_path(&reexported_at),
                    rules::method_visibility::MethodVisibility::CODE,
                    format!(
                        "method file `{}` of struct `{}` is re-exported with `pub use` — remove the re-export",
                        file_name, struct_name
                    ),
                ));
            }
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use crate::rules::method_visibility::MethodVisibility;
    use crate::testing;
    use crate::Entry;
    use crate::EntryKind;
    use crate::ModuleInfo;
    use crate::Project;
    use std::collections::HashSet;
    use std::path::PathBuf;

    use super::rule_check;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let mod_abs = dir.path().join("mod.rs");
        std::fs::write(&mod_abs, "pub mod player_new;\n").unwrap();
        let entries = vec![
            Entry {
                relative_path: PathBuf::from("player/player.rs"),
                absolute_path: PathBuf::from("player/player.rs"),
                kind: EntryKind::File,
            },
            Entry {
                relative_path: PathBuf::from("player/player_new.rs"),
                absolute_path: PathBuf::from("player/player_new.rs"),
                kind: EntryKind::File,
            },
            Entry {
                relative_path: PathBuf::from("player/mod.rs"),
                absolute_path: mod_abs,
                kind: EntryKind::File,
            },
        ];
        let mut project = testing::parsed_project(
            Project::default(),
            &[
                ("player/player.rs", "pub struct Player;"),
                ("player/player_new.rs", "pub fn new() {}"),
            ],
        );
        project.module_info = ModuleInfo::build(&entries);
        project.entries = entries;
        let violations = rule_check(&MethodVisibility, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("declared `pub mod`"));
    }

    #[test]
    fn method_stem_resolves_the_longest_type_prefix() {
        let mut struct_names = HashSet::new();
        struct_names.insert("player".to_string());
        struct_names.insert("config".to_string());
        struct_names.insert("freenet_client".to_string());

        assert_eq!(
            testing::method_struct("player_new.rs", &struct_names),
            Some("player".to_string())
        );
        assert_eq!(
            testing::method_struct("freenet_client_connect.rs", &struct_names),
            Some("freenet_client".to_string())
        );
        assert_eq!(
            testing::method_struct("bevy_systems.rs", &struct_names),
            None
        );
    }
}
