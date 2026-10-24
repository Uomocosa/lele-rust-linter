use syn::visit::Visit;

use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check(
    _self: &rules::no_super_imports::NoSuperImports,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for source in project.content_sources() {
        let rel_path = source.relative_path;
        let file = source.file;
        let mut hits = Vec::new();
        {
            let mut visitor = rules::no_super_imports::SuperPathVisitor(&mut hits);
            visitor.visit_file(file);
        }
        for hit in hits {
            let (import, replacement) =
                rules::no_super_imports::NoSuperImports::suggestion(rel_path, &hit.path);
            violations.push(RuleViolation::from_site(project.absolute_path(source.origin, rel_path), hit.line, 0, rules::no_super_imports::NoSuperImports::CODE, format!(
                    "`{}` used outside `#[cfg(test)]` — write `{replacement}` instead and add `{import}` at the top of the file",
                    hit.path
                )));
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use std::path::PathBuf;

    use super::check;
    use crate::rules::no_super_imports::NoSuperImports;
    use crate::Project;
    use syn::visit::Visit;

    fn hit_lines(src: &str) -> Vec<usize> {
        let file = syn::parse_file(src).unwrap();
        let mut hits = Vec::new();
        {
            let mut visitor = rules::no_super_imports::SuperPathVisitor(&mut hits);
            visitor.visit_file(&file);
        }
        hits.into_iter().map(|hit| hit.line).collect()
    }

    #[test]
    fn test_usage() {
        assert_eq!(hit_lines("use super::config_load;\n"), vec![1]);
        assert_eq!(hit_lines("use super::super::Event;\n"), vec![1]);
        assert_eq!(hit_lines("fn f() { super::a::b(); }\n"), vec![1]);
    }

    #[test]
    fn test_usage_allows_super_in_cfg_test_mod() {
        assert_eq!(
            hit_lines("#[cfg(test)]\nmod tests { use super::drain_events; }\n"),
            Vec::<usize>::new()
        );
    }

    #[test]
    fn test_usage_allows_super_in_cfg_test_fn() {
        assert_eq!(
            hit_lines("#[cfg(test)]\nfn helper() { super::a::b(); }\n"),
            Vec::<usize>::new()
        );
    }

    #[test]
    fn test_usage_message_names_domain_fix() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("rules/probe.rs"),
            syn::parse_str("use super::probe_check;\n").unwrap(),
        );
        let violations = check(&NoSuperImports, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("use crate::rules;"));
        assert!(violations[0].message.contains("rules::probe_check"));
    }

    #[test]
    fn test_usage_message_names_root_fix() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("probe.rs"),
            syn::parse_str("use super::sibling;\n").unwrap(),
        );
        let violations = check(&NoSuperImports, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("use crate::sibling;"));
    }
}

// no test_usage necessary
