use std::collections::BTreeMap;
use std::path::Path;

use crate::common;
use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check_root_index(
    project: &Project,
    declared: &BTreeMap<String, common::DeclaredType>,
) -> Vec<RuleViolation> {
    let root = project.methods_root_name();
    let root_mod = Path::new("mod.rs");
    let expected = common::DeclaredType::root_index_content(declared);
    if project.methods_parsed_files.contains_key(root_mod) {
        if rules::methods_layout::MethodsLayout::read_methods_file(project, root_mod).as_deref()
            != Some(expected.as_str())
        {
            return vec![rules::methods_layout::MethodsLayout::file_violation(
                project,
                root_mod,
                format!("`{root}/mod.rs` is out of date — run `lele-lint --sync-methods`"),
            )];
        }
        return Vec::new();
    }
    vec![rules::methods_layout::MethodsLayout::file_violation(
        project,
        root_mod,
        format!("missing `{root}/mod.rs` — run `lele-lint --sync-methods`"),
    )]
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::check_root_index;
    use crate::Project;

    #[test]
    fn test_usage() {
        let violations = check_root_index(&Project::default(), &BTreeMap::new());
        assert_eq!(violations.len(), 1);
        assert!(violations[0]
            .message
            .contains("missing `__methods__/mod.rs`"));
    }
}
