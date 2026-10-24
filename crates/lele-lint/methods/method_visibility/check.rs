use crate::rules;
use crate::Identifier;
use crate::Project;
use crate::RuleViolation;

pub fn check(
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
                violations.push(RuleViolation::from_file(declared_pub, rules::method_visibility::MethodVisibility::CODE, format!(
                        "method file `{}` of struct `{}` must be declared with `mod` (private), not `pub mod`",
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
                    reexported_at,
                    rules::method_visibility::MethodVisibility::CODE,
                    format!(
                        "method file `{}` of struct `{}` must not appear in a `pub use` re-export",
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
    use crate::Identifier;
    use std::collections::HashSet;

    fn method_struct(file_name: &str, struct_names: &HashSet<String>) -> Option<String> {
        file_name
            .strip_suffix(".rs")
            .and_then(|stem| Identifier(stem).longest_type_prefix(struct_names))
            .map(str::to_string)
    }

    #[test]
    fn test_usage() {
        let mut struct_names = HashSet::new();
        struct_names.insert("player".to_string());
        struct_names.insert("config".to_string());
        struct_names.insert("freenet_client".to_string());

        assert_eq!(
            method_struct("player_new.rs", &struct_names),
            Some("player".to_string())
        );
        assert_eq!(
            method_struct("freenet_client_connect.rs", &struct_names),
            Some("freenet_client".to_string())
        );
        assert_eq!(method_struct("bevy_systems.rs", &struct_names), None);
    }
}
