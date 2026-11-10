use syn::visit::Visit;

use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check(_self: &rules::no_inner_fns::NoInnerFns, project: &Project) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for source in project.content_sources() {
        let mut finder = rules::no_inner_fns::InnerFnFinder::default();
        finder.visit_file(source.file);
        let absolute = project.absolute_path(source.origin, source.relative_path);
        violations.extend(finder.inner_fns.iter().map(|inner| {
            RuleViolation::from_site(
                absolute.clone(),
                inner.sig.fn_token.span.start().line,
                0,
                rules::no_inner_fns::NoInnerFns::CODE,
                format!(
                    "inner fn `{}` — only `fn recursion` may live inside a function; make it a `pub(crate)` method or associated fn of the type",
                    inner.sig.ident
                ),
            )
        }));
    }
    violations
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::check;
    use crate::rules::no_inner_fns::NoInnerFns;
    use crate::Project;

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("tree.rs"),
            syn::parse_str("fn total() { fn recursion() {} fn type_name() {} }").unwrap(),
        );
        let messages: Vec<String> = check(&NoInnerFns, &project)
            .into_iter()
            .map(|v| v.message)
            .collect();
        assert_eq!(messages.len(), 1);
        assert!(messages
            .iter()
            .all(|m| m.starts_with("inner fn `type_name`")));
    }
}
