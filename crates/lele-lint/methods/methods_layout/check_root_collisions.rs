use std::collections::BTreeSet;
use std::path::PathBuf;

use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check_root_collisions(project: &Project) -> Vec<RuleViolation> {
    let names = project.layout_root_names();
    let Some((_, extra)) = names.split_first() else {
        return Vec::new();
    };
    let mut rels: Vec<&PathBuf> = project.parsed_files.keys().collect();
    rels.sort();
    let mut reported: BTreeSet<String> = BTreeSet::new();
    let mut violations = Vec::new();
    for rel in rels {
        if project.src_index_for_rel(rel) != 0 {
            continue;
        }
        let mut components = rel.components();
        let Some(first) = components.next().and_then(|c| c.as_os_str().to_str()) else {
            continue;
        };
        let colliding = if components.next().is_some() {
            extra
                .iter()
                .any(|name| name == first)
                .then(|| first.to_string())
        } else {
            rel.file_stem()
                .and_then(|stem| stem.to_str())
                .filter(|stem| extra.iter().any(|name| name == stem))
                .map(str::to_string)
        };
        let Some(name) = colliding else {
            continue;
        };
        if reported.insert(name.clone()) {
            violations.push(RuleViolation::from_file(
                project.absolute_src_path(rel),
                rules::methods_layout::MethodsLayout::CODE,
                format!(
                    "`{}` collides with source root `{name}` — no top-level path under the first root may share its name with another source root; rename the file or folder, or the root",
                    rel.display()
                ),
            ));
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::check_root_collisions;
    use crate::Project;

    #[test]
    fn test_usage() {
        let mut project = Project {
            layout_roots: vec![PathBuf::from("src"), PathBuf::from("ui")],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("widget.rs"),
            syn::parse_str("pub struct Widget;").unwrap(),
        );
        assert!(check_root_collisions(&project).is_empty());
        project.parsed_files.insert(
            PathBuf::from("ui/panel.rs"),
            syn::parse_str("pub struct Panel;").unwrap(),
        );
        let violations = check_root_collisions(&project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0]
            .message
            .contains("collides with source root `ui`"));
    }
}
