use syn::spanned::Spanned;

use crate::checkers;
use crate::common;
use crate::Project;
use crate::RuleViolation;

pub fn check(
    _self: &checkers::test_is_not_helper::TestIsNotHelper,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for source in project.content_sources() {
        let absolute = project.absolute_path(source.origin, source.relative_path);
        let Ok(text) = std::fs::read_to_string(&absolute) else {
            continue;
        };
        let lines: Vec<&str> = text.lines().collect();
        for item in &source.file.items {
            let syn::Item::Fn(func) = item else {
                continue;
            };
            if !common::has_attr(&func.attrs, "test") {
                continue;
            }
            let line = func.sig.fn_token.span().start().line;
            if !common::annotated_as_helper(&lines, line) {
                continue;
            }
            violations.push(RuleViolation::site(
                absolute.clone(),
                line,
                0,
                checkers::test_is_not_helper::TestIsNotHelper::CODE,
                "`#[test]` fns are not helpers — drop the `// needed helper:` line".to_string(),
            ));
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use super::check;
    use crate::checkers;
    use crate::Project;

    #[test]
    fn test_usage() {
        let checker = checkers::test_is_not_helper::TestIsNotHelper;
        let project = Project::default();
        assert!(check(&checker, &project).is_empty());
    }
}
