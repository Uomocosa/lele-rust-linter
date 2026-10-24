use crate::rules;
use crate::Origin;
use crate::Project;
use crate::RuleViolation;

const OPT_OUT: &str = "// no test_usage necessary";

pub fn check(_self: &rules::test_usage::TestUsage, project: &Project) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for source in project.content_sources() {
        if matches!(source.origin, Origin::Examples) {
            continue;
        }
        let rel_path = source.relative_path;
        if rules::test_usage::TestUsage::is_exempt(rel_path, source.file, project) {
            continue;
        }

        let absolute = project.absolute_path(source.origin, rel_path);
        if rules::test_usage::TestUsage::reads_opt_out(&absolute) {
            continue;
        }

        if !source.has_test_usage() {
            violations.push(RuleViolation::from_file(absolute, rules::test_usage::TestUsage::CODE, format!(
                    "file `{}` must contain a `#[cfg(test)] mod tests {{ fn test_usage() {{ ... }} }}` block, or add `{OPT_OUT}` as its last line to opt out",
                    rel_path.display()
                )));
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::check;
    use crate::rules;
    use crate::Origin;
    use crate::ParsedSource;
    use crate::Project;
    use std::path::Path;

    #[test]
    fn test_usage() {
        let rule = rules::test_usage::TestUsage;
        let project = Project::default();
        assert!(check(&rule, &project).is_empty());
    }

    #[test]
    fn test_usage_finds_test_usage() {
        let file: syn::File =
            syn::parse_str("#[cfg(test)] mod tests { #[test] fn test_usage() { assert!(true); } }")
                .unwrap();
        assert!(ParsedSource {
            origin: Origin::Src,
            relative_path: Path::new("a.rs"),
            file: &file
        }
        .has_test_usage());
    }

    #[test]
    fn test_usage_missing() {
        let file: syn::File = syn::parse_str("pub fn compute(x: u32) -> u32 { x * 2 }").unwrap();
        assert!(!ParsedSource {
            origin: Origin::Src,
            relative_path: Path::new("a.rs"),
            file: &file
        }
        .has_test_usage());
    }

    #[test]
    fn test_usage_opt_out_only_at_end() {
        assert!(!rules::test_usage::TestUsage::opt_out_at_end(
            "// no test_usage necessary\n\npub fn compute() {}\n"
        ));
        assert!(rules::test_usage::TestUsage::opt_out_at_end(
            "pub fn compute() {}\n\n// no test_usage necessary\n"
        ));
        assert!(!rules::test_usage::TestUsage::opt_out_at_end(
            "pub fn compute() {}\n"
        ));
    }
}

// no test_usage necessary
