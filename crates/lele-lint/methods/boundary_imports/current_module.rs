use std::path::Path;

use crate::common;
use crate::Project;

pub fn current_module(project: &Project, crate_rel: &Path) -> Vec<String> {
    if let Ok(rest) = crate_rel.strip_prefix("src") {
        let mut module = vec!["crate".to_string()];
        module.extend(common::SourcePath(rest).module_path());
        return module;
    }
    if let Ok(rest) = crate_rel.strip_prefix("methods") {
        let (_, inner) = project.split_methods_rel(rest);
        let mut module = vec!["crate".to_string(), "methods".to_string()];
        module.extend(common::SourcePath(&inner).module_path());
        return module;
    }
    let mut module = vec!["crate".to_string()];
    module.extend(common::SourcePath(crate_rel).module_path());
    module
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;

    use super::current_module;
    use crate::Project;

    #[test]
    fn test_usage() {
        let project = Project::default();
        assert_eq!(
            current_module(&project, Path::new("src/session/session.rs")),
            vec![
                "crate".to_string(),
                "session".to_string(),
                "session".to_string()
            ]
        );
        assert_eq!(
            current_module(&project, Path::new("src/lib.rs")),
            vec!["crate".to_string()]
        );
        let multi = Project {
            layout_roots: vec![PathBuf::from("src"), PathBuf::from("ui")],
            ..Project::default()
        };
        assert_eq!(
            current_module(&multi, Path::new("methods/src/widget/spin.rs")),
            vec![
                "crate".to_string(),
                "methods".to_string(),
                "widget".to_string(),
                "spin".to_string()
            ]
        );
        assert_eq!(
            current_module(&multi, Path::new("ui/discovery.rs")),
            vec![
                "crate".to_string(),
                "ui".to_string(),
                "discovery".to_string()
            ]
        );
    }
}
