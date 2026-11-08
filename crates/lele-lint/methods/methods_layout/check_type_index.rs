use std::path::Path;

use crate::common;
use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check_type_index(
    project: &Project,
    type_snake: &str,
    declared_type: &common::DeclaredType,
) -> Vec<RuleViolation> {
    let mod_rel = Path::new(type_snake).join("mod.rs");
    let expected = declared_type.index_content();
    if project.methods_parsed_files.contains_key(&mod_rel) {
        if rules::methods_layout::MethodsLayout::read_methods_file(project, &mod_rel).as_deref()
            != Some(expected.as_str())
        {
            return vec![rules::methods_layout::MethodsLayout::file_violation(
                project,
                &mod_rel,
                format!(
                    "`methods/{type_snake}/mod.rs` is out of date — run `lele-lint --sync-methods`"
                ),
            )];
        }
        return Vec::new();
    }
    vec![rules::methods_layout::MethodsLayout::file_violation(
        project,
        &mod_rel,
        format!("missing `methods/{type_snake}/mod.rs` — run `lele-lint --sync-methods`"),
    )]
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::PathBuf;

    use super::check_type_index;
    use crate::common::DeclaredType;
    use crate::Project;

    #[test]
    fn test_usage() {
        let project = Project::default();
        let declared = DeclaredType {
            methods: BTreeSet::from(["spin".to_string()]),
            cfgs: Vec::new(),
        };
        let violations = check_type_index(&project, "widget", &declared);
        assert_eq!(violations.len(), 1);
        assert!(violations[0]
            .message
            .contains("missing `methods/widget/mod.rs`"));
        let mut with_mod = Project::default();
        with_mod.methods_parsed_files.insert(
            PathBuf::from("widget/mod.rs"),
            syn::parse_str("mod spin;\npub use spin::spin;\n").unwrap(),
        );
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("widget")).unwrap();
        std::fs::write(
            dir.path().join("widget").join("mod.rs"),
            "mod spin;\npub use spin::spin;\n",
        )
        .unwrap();
        with_mod.methods_dir = Some(dir.path().to_path_buf());
        with_mod.methods_entries = vec![crate::Entry {
            relative_path: PathBuf::from("widget/mod.rs"),
            absolute_path: dir.path().join("widget").join("mod.rs"),
            kind: crate::EntryKind::File,
        }];
        assert!(check_type_index(&with_mod, "widget", &declared).is_empty());
    }
}
