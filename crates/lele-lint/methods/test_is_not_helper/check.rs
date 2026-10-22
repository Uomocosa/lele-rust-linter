use crate::checkers;
use crate::ItemMarker;
use crate::Project;
use crate::RawCode;
use crate::RuleViolation;
use syn::spanned::Spanned;

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
        for item in &source.file.items {
            let syn::Item::Fn(func) = item else {
                continue;
            };
            if !func.attrs.iter().any(|attr| ItemMarker(attr).is_test()) {
                continue;
            }
            let line = func.sig.fn_token.span().start().line;
            if !RawCode(&text).is_annotated_as_helper(line) {
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
