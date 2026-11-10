use syn::visit::Visit;

use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check(_self: &rules::no_positional::NoPositional, project: &Project) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for source in project.content_sources() {
        let file_path = project.absolute_path(source.origin, source.relative_path);
        let mut visitor = rules::no_positional::PositionalVisitor {
            file_path,
            violations: Vec::new(),
        };
        visitor.visit_file(source.file);
        violations.extend(visitor.violations);
    }
    violations
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use syn::visit::Visit;

    use crate::rules;
    use crate::RuleViolation;

    fn violations_for(source: &str) -> Vec<RuleViolation> {
        let file: syn::File = syn::parse_str(source).unwrap();
        let mut visitor = rules::no_positional::PositionalVisitor {
            file_path: PathBuf::from("probe.rs"),
            violations: Vec::new(),
        };
        visitor.visit_file(&file);
        visitor.violations
    }

    #[test]
    fn test_usage() {
        let violations = violations_for(
            "pub struct Pos(pub String, pub u32);\npub fn f(p: &Pos) -> &String { &p.0 }\n",
        );
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_usage_catches_nested_access() {
        let violations = violations_for(
            "pub struct Pos(pub String);\npub fn f(p: &Pos) -> String { g(p.0, h().1) }\n",
        );
        assert_eq!(violations.len(), 2);
    }

    #[test]
    fn test_usage_ignores_floats_in_macros() {
        let violations =
            violations_for("pub fn f() -> f64 { let x = 0.0; println!(\"{x}\"); x }\n");
        assert!(violations.is_empty());
    }
}
