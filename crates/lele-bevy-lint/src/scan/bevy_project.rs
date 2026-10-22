use std::collections::BTreeSet;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;
use lele_lint::Origin;
use lele_lint::Project;
use lele_lint::RuleViolation;

#[derive(Clone, Copy, Deref)]
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
