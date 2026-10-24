use lele_snake_case::ToLeleSnakeCase;

use crate::rules;
use crate::Identifier;
use crate::Project;
use crate::RuleViolation;

pub fn check(
    _self: &rules::no_stuttered_type::NoStutteredType,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for source in project.content_sources() {
        let rel_path = source.relative_path;
        let file = source.file;
        if project.is_exempt_source_path(rel_path) {
            continue;
        }
        let Some(dir) = rules::no_stuttered_type::NoStutteredType::parent_dir_name(rel_path) else {
            continue;
        };
        let stem = rel_path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        let Some(name) =
            rules::no_stuttered_type::NoStutteredType::primary_exposed_type(file, stem)
        else {
            continue;
        };
        let Some(suffix) = rules::no_stuttered_type::NoStutteredType::strip_dir_prefix(
            &name.to_snake_case(),
            &dir,
        ) else {
            continue;
        };
        let suggested = Identifier(&suffix).to_pascal_case();
        violations.push(RuleViolation::from_file(project.absolute_path(source.origin, rel_path), rules::no_stuttered_type::NoStutteredType::CODE, format!(
                "type `{name}` repeats parent module `{dir}` — rename to `{suggested}` (`{dir}::{suggested}`)"
            )));
    }

    violations
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use std::collections::HashMap;
    use std::path::PathBuf;

    use crate::rules::no_stuttered_type::NoStutteredType;
    use crate::Project;

    fn project_with(files: Vec<(&str, &str)>) -> Project {
        let parsed_files = files
            .into_iter()
            .map(|(path, code)| {
                (
                    PathBuf::from(path),
                    syn::parse_str(code).expect("bad fixture"),
                )
            })
            .collect::<HashMap<_, _>>();
        Project {
            root: PathBuf::from("."),
            src_dir: PathBuf::from("src"),
            entries: Vec::new(),
            module_info: HashMap::default(),
            parsed_files,
            ..Project::default()
        }
    }

    #[test]
    fn test_usage() {
        let project = project_with(vec![(
            "freenet/freenet_client.rs",
            "pub struct FreenetClient(pub String);",
        )]);
        let violations = super::check(&NoStutteredType, &project);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].code, NoStutteredType::CODE);
        assert!(violations[0].message.contains("`Client`"));
    }

    #[test]
    fn test_usage_exact_match_is_exempt() {
        let project = project_with(vec![
            ("cli/cli.rs", "pub struct Cli;"),
            ("roster/roster.rs", "pub struct Roster;"),
        ]);
        let violations = super::check(&NoStutteredType, &project);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_non_prefix_is_exempt() {
        let project = project_with(vec![
            ("p2p/event.rs", "pub enum Event<T> { A(T) }"),
            ("plugin/p2p_plugin.rs", "pub struct P2PPlugin;"),
            ("relay/letter_request.rs", "pub struct LetterRequest;"),
        ]);
        let violations = super::check(&NoStutteredType, &project);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_short_suffix_is_exempt() {
        let project = project_with(vec![(
            "net_id/network_id.rs",
            "pub struct NetworkId(pub u64);",
        )]);
        let violations = super::check(&NoStutteredType, &project);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_root_and_private_are_exempt() {
        let project = project_with(vec![
            ("freenet_client.rs", "pub struct FreenetClient;"),
            ("freenet/hidden.rs", "struct Hidden;"),
        ]);
        let violations = super::check(&NoStutteredType, &project);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_strip_dir_prefix() {
        assert_eq!(
            rules::no_stuttered_type::NoStutteredType::strip_dir_prefix(
                "freenet_client",
                "freenet"
            ),
            Some("client".to_string())
        );
        assert_eq!(
            rules::no_stuttered_type::NoStutteredType::strip_dir_prefix("p2p_plugin", "plugin"),
            None
        );
        assert_eq!(
            rules::no_stuttered_type::NoStutteredType::strip_dir_prefix("cli", "cli"),
            None
        );
        assert_eq!(
            rules::no_stuttered_type::NoStutteredType::strip_dir_prefix("network_id", "net_id"),
            None
        );
    }
}
