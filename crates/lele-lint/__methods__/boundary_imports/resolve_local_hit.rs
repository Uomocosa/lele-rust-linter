use std::path::Path;

use crate::rules;
use crate::Project;

pub fn resolve_local_hit(project: &Project, crate_rel: &Path, hit: &str) -> Option<String> {
    if hit == "crate" || hit.starts_with("crate::") {
        return Some(hit.to_string());
    }
    let module = rules::boundary_imports::BoundaryImports::current_module(project, crate_rel);
    if hit == "self" {
        return Some(module.join("::"));
    }
    if let Some(rest) = hit.strip_prefix("self::") {
        return Some(format!("{}::{rest}", module.join("::")));
    }
    let supers = hit
        .split("::")
        .take_while(|segment| *segment == "super")
        .count();
    if supers > 0 {
        let rest = hit.split("::").skip(supers).collect::<Vec<_>>().join("::");
        if supers >= module.len() {
            return None;
        }
        let head = module.get(..module.len().saturating_sub(supers))?;
        let parent = head.join("::");
        if rest.is_empty() {
            return Some(parent);
        }
        return Some(format!("{parent}::{rest}"));
    }
    None
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;

    use super::resolve_local_hit;
    use crate::Project;

    #[test]
    fn test_usage() {
        let project = Project::default();
        let rel = Path::new("src/session/session.rs");
        assert_eq!(
            resolve_local_hit(&project, rel, "crate::ui::Thing"),
            Some("crate::ui::Thing".to_string())
        );
        assert_eq!(
            resolve_local_hit(&project, rel, "self::helper"),
            Some("crate::session::session::helper".to_string())
        );
        assert_eq!(
            resolve_local_hit(&project, rel, "super::sibling"),
            Some("crate::session::sibling".to_string())
        );
        assert_eq!(resolve_local_hit(&project, rel, "std::time"), None);
    }

    #[test]
    fn test_usage_extra_root_resolves_to_own_module() {
        let project = Project {
            layout_roots: vec![PathBuf::from("src"), PathBuf::from("ui")],
            ..Project::default()
        };
        let rel = Path::new("ui/discovery.rs");
        assert_eq!(
            resolve_local_hit(&project, rel, "super::sibling"),
            Some("crate::ui::sibling".to_string())
        );
    }

    #[test]
    fn test_usage_too_many_supers_is_none() {
        let project = Project::default();
        assert_eq!(
            resolve_local_hit(&project, Path::new("src/lib.rs"), "super::x"),
            None
        );
    }
}
