use syn::visit::Visit;

use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check(
    _self: &rules::dunder_module_name::DunderModuleName,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for source in project.content_sources() {
        let mut hits = Vec::new();
        {
            let mut visitor = rules::dunder_module_name::DunderNameVisitor {
                dunders: &project.dunder_paths,
                hits: &mut hits,
            };
            visitor.visit_file(source.file);
        }
        for hit in hits {
            violations.push(RuleViolation::from_site(
                project.absolute_path(source.origin, source.relative_path),
                hit.line,
                0,
                rules::dunder_module_name::DunderModuleName::CODE,
                hit.message,
            ));
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::check;
    use crate::rules::dunder_module_name::DunderModuleName;
    use crate::DunderPath;
    use crate::Project;

    fn messages(src: &str) -> Vec<String> {
        let mut project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        project
            .parsed_files
            .insert(PathBuf::from("stock/mod.rs"), syn::parse_str(src).unwrap());
        check(&DunderModuleName, &project)
            .into_iter()
            .map(|violation| violation.message)
            .collect()
    }

    #[test]
    fn test_usage() {
        assert_eq!(
            messages("#[path = \"__basic__/mod.rs\"]\npub mod basic;\n"),
            Vec::<String>::new()
        );
        let found = messages("#[path = \"__basic__/mod.rs\"]\npub mod __basic__;\n");
        assert_eq!(found.len(), 1);
        assert!(found[0].contains("pub mod basic;"));
    }

    #[test]
    fn test_usage_flags_dunder_mod_without_path() {
        let found = messages("pub mod __basic__;\n");
        assert_eq!(found.len(), 1);
        assert!(found[0].contains("#[path = \"__basic__/mod.rs\"]"));
    }

    #[test]
    fn test_usage_flags_path_to_dunder_with_other_name() {
        let found = messages("#[path = \"__basic__/mod.rs\"]\npub mod helpers;\n");
        assert_eq!(found.len(), 1);
        assert!(found[0].contains("name the module `basic`, not `helpers`"));
    }

    #[test]
    fn test_usage_allows_path_to_file_inside_dunder() {
        assert_eq!(
            messages("#[path = \"__basic__/enums.rs\"]\nmod enums;\n"),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_usage_flags_dunder_path_segments() {
        let src = "use crate::__basic__;\nuse crate::stock::__basic__::enums;\n\
                   fn f(level: __basic__::enums::Bump) {}\n";
        assert_eq!(messages(src).len(), 3);
    }

    #[test]
    fn test_usage_ignores_unconfigured_dunders() {
        assert_eq!(
            messages("pub mod __other__;\nuse crate::__other__;\n"),
            Vec::<String>::new()
        );
    }
}

// no test_usage necessary
