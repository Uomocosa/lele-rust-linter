use std::path::PathBuf;

use crate::methods;
use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub(crate) fn atomic_delegates_violations(src: &str) -> Vec<RuleViolation> {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("foo.rs"), src).unwrap();
    let mut project = Project {
        root: dir.path().to_path_buf(),
        src_dir: dir.path().to_path_buf(),
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
    use super::atomic_delegates_violations;

    #[test]
    fn test_usage() {
        let violations = atomic_delegates_violations(
            "pub struct Foo;\nimpl Foo { fn name(&self) -> &'static str { Self::NAME } }\n",
        );
        assert_eq!(violations.len(), 1);
    }
}
