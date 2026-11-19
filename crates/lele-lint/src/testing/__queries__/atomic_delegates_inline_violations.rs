use std::path::PathBuf;

use crate::methods;
use crate::rules;
use crate::InlineTraitMethodsEntry;
use crate::Project;
use crate::RuleViolation;

pub(crate) fn atomic_delegates_inline_violations(
    src: &str,
    trait_path: &str,
    methods: &[&str],
) -> Vec<RuleViolation> {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("foo.rs"), src).unwrap();
    let mut project = Project {
        root: dir.path().to_path_buf(),
        src_dir: dir.path().to_path_buf(),
        inline_trait_methods: vec![InlineTraitMethodsEntry {
            trait_path: trait_path.to_string(),
            methods: methods.iter().map(|method| (*method).to_string()).collect(),
            reason: "listed by the test".to_string(),
        }],
        ..Project::default()
    };
    project
        .parsed_files
        .insert(PathBuf::from("foo.rs"), syn::parse_str(src).unwrap());
    project.fill_rustfmt_answers().unwrap();
    methods::atomic_delegates::rule_check(&rules::atomic_delegates::AtomicDelegates, &project)
}

#[cfg(test)]
mod tests {
    use super::atomic_delegates_inline_violations;

    #[test]
    fn test_usage() {
        let violations = atomic_delegates_inline_violations(
            "pub struct Foo;\nimpl Rule for Foo { fn run(&self) -> u8 { self.0 } }\n",
            "crate::Rule",
            &["run"],
        );
        assert!(violations.is_empty());
    }
}
