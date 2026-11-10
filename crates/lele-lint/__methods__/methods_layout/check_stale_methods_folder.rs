use crate::rules;
use crate::Entry;
use crate::EntryKind;
use crate::Project;
use crate::RuleViolation;

const STALE_FOLDER_NAME: &str = "methods";
const DEFAULT_FOLDER_NAME: &str = "__methods__";

pub fn check_stale_methods_folder(project: &Project) -> Vec<RuleViolation> {
    let stale = project.root.join(STALE_FOLDER_NAME);
    let Ok(entries) = Entry::walk(&stale, &stale) else {
        return Vec::new();
    };
    let configured = project
        .methods_dir
        .as_ref()
        .is_some_and(|methods_dir| *methods_dir == stale);
    if configured
        || !entries.iter().any(|entry| {
            entry.kind == EntryKind::File
                && entry
                    .relative_path
                    .extension()
                    .is_some_and(|ext| ext == "rs")
        })
    {
        return Vec::new();
    }
    vec![RuleViolation::from_file(
        stale,
        rules::methods_layout::MethodsLayout::CODE,
        format!(
            "crate-root folder `{STALE_FOLDER_NAME}/` holds delegate bodies — rename `{STALE_FOLDER_NAME}/` to `{DEFAULT_FOLDER_NAME}/` and update the `#[path]` mount"
        ),
    )]
}

#[cfg(test)]
mod tests {
    use super::check_stale_methods_folder;
    use crate::Project;

    fn project_with_methods_root(methods_root: Option<&str>) -> (tempfile::TempDir, Project) {
        let dir = tempfile::tempdir().unwrap();
        let folder = dir.path().join(super::STALE_FOLDER_NAME);
        std::fs::create_dir_all(&folder).unwrap();
        std::fs::write(folder.join("mod.rs"), "pub mod player;\n").unwrap();
        let mut project = Project {
            root: dir.path().to_path_buf(),
            ..Project::default()
        };
        if let Some(methods_root) = methods_root {
            project.methods_dir = Some(dir.path().join(methods_root));
        }
        (dir, project)
    }

    #[test]
    fn test_usage() {
        let (_dir, project) = project_with_methods_root(None);
        let violations = check_stale_methods_folder(&project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].file.ends_with(super::STALE_FOLDER_NAME));
        assert_eq!(violations[0].code, "LELE030");
        assert!(violations[0]
            .message
            .contains("rename `methods/` to `__methods__/` and update the `#[path]` mount"));
    }

    #[test]
    fn test_usage_skip_when_folder_is_the_configured_methods_root() {
        let (_dir, project) = project_with_methods_root(Some(super::STALE_FOLDER_NAME));
        assert!(check_stale_methods_folder(&project).is_empty());
    }

    #[test]
    fn test_usage_empty_folder_reports_nothing() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join(super::STALE_FOLDER_NAME)).unwrap();
        let project = Project {
            root: dir.path().to_path_buf(),
            ..Project::default()
        };
        assert!(check_stale_methods_folder(&project).is_empty());
    }

    #[test]
    fn test_usage_no_folder_reports_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let project = Project {
            root: dir.path().to_path_buf(),
            ..Project::default()
        };
        assert!(check_stale_methods_folder(&project).is_empty());
    }

    #[test]
    fn test_usage_marker_files_without_rs_report_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let folder = dir.path().join(super::STALE_FOLDER_NAME);
        std::fs::create_dir_all(&folder).unwrap();
        std::fs::write(folder.join("notes.md"), "x").unwrap();
        let project = Project {
            root: dir.path().to_path_buf(),
            ..Project::default()
        };
        assert!(check_stale_methods_folder(&project).is_empty());
    }
}
