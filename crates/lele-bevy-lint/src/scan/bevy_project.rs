use std::collections::BTreeSet;
use std::collections::HashSet;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;
use lele_lint::Origin;
use lele_lint::Project;
use lele_lint::RuleViolation;

use crate::scan;

#[derive(Clone, Deref)]
pub struct BevyProject<'a>(pub &'a Project);

#[atomic_delegates]
impl BevyProject<'_> {
    pub fn require_visuals(&self) -> BTreeSet<String> {}
    pub fn test_only_files(&self) -> HashSet<PathBuf> {}
    pub fn dir_spawns_ui(&self, origin: Origin, rel_path: &Path) -> bool {}
    pub fn registers_ui(&self, registered: &[Vec<String>]) -> bool {}
    pub fn violation(
        &self,
        origin: Origin,
        rel_path: &Path,
        line: usize,
        code: &'static str,
        message: String,
    ) -> RuleViolation {
    }
    pub fn ui_inventory(&self) -> String {}
}

#[atomic_delegates]
impl BevyProject<'_> {
    pub(crate) fn file_spawns_ui(
        project: &BevyProject,
        segments: &[String],
        extra: &BTreeSet<String>,
    ) -> bool {
    }
    pub(crate) fn module_files(declaring: &Path, module: &syn::ItemMod) -> Vec<PathBuf> {}
    pub(crate) fn rows(project: &BevyProject) -> Vec<scan::Row> {}
    pub(crate) fn path_attr(attrs: &[syn::Attribute]) -> Option<String> {}
    pub(crate) fn requires_visual(attrs: &[syn::Attribute]) -> bool {}
}

#[rustfmt::skip]
impl BevyProject<'_> {
    pub(crate) fn display_path(origin: Origin, rel_path: &Path) -> String {
        match origin {
            Origin::Src => rel_path.display().to_string(),
            Origin::Methods => format!("methods/{}", rel_path.display()),
            Origin::Examples => format!("examples/{}", rel_path.display()),
        }
    }
    pub(crate) fn mark(present: bool) -> &'static str {
        if present { "x" } else { "-" }
    }
}

impl BevyProject<'_> {
    pub(crate) fn normalize(path: &Path) -> PathBuf {
        let mut out = PathBuf::new();
        for part in path.components() {
            match part {
                Component::ParentDir => {
                    out.pop();
                }
                Component::CurDir => {}
                other => out.push(other),
            }
        }
        out
    }
    pub(crate) fn join_unique(mut items: Vec<String>) -> String {
        items.sort();
        items.dedup();
        items.join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::BevyProject;
    use lele_lint::Project;

    #[test]
    fn test_usage() {
        let project = Project::default();
        assert!(BevyProject(&project).require_visuals().is_empty());
    }
}
