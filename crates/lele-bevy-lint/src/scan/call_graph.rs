use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

use atomic_delegate_macros::atomic_delegates;
use lele_lint::Project;

use crate::scan;

pub struct CallGraph {
    pub(crate) callees: HashMap<String, Vec<String>>,
    pub(crate) systems: HashSet<String>,
}

impl From<&Project> for CallGraph {
    fn from(project: &Project) -> Self {
        let mut graph = Self {
            callees: HashMap::new(),
            systems: HashSet::new(),
        };
        graph.visit_production(project);
        graph
    }
}

#[rustfmt::skip]
impl CallGraph {
    #[must_use]
    pub fn reaches_production(&self, owner: &str) -> bool {
        if owner.is_empty() {
            return true;
        }
        let targets = BTreeSet::from([owner.to_string()]);
        self.reachable(&self.entry_roots(), &targets)
    }

    pub(crate) fn entry_roots(&self) -> Vec<String> {
        let mut roots = vec!["main".to_string(), "build".to_string(), "setup".to_string()];
        roots.extend(self.systems.iter().cloned());
        roots
    }
    pub(crate) fn visit_production(&mut self, project: &Project) {
        let test_only = scan::BevyProject(project).test_only_files();
        for source in project.sources() {
            if test_only.contains(source.relative_path) {
                continue;
            }
            for item in &source.file.items {
                if !scan::ScannedItem(item).is_test_module() {
                    self.visit_item(item);
                }
            }
        }
    }

    pub(crate) fn visit_item(&mut self, item: &syn::Item) {
        match item {
            syn::Item::Fn(func) => {
                self.record_call(&func.sig.ident.to_string(), &func.block);
            }
            syn::Item::Impl(impl_block) => {
                for item in &impl_block.items {
                    if let syn::ImplItem::Fn(method) = item {
                        self.record_call(&method.sig.ident.to_string(), &method.block);
                    }
                }
            }
            syn::Item::Mod(module) => {
                if let Some((_, inner)) = &module.content {
                    for item in inner {
                        self.visit_item(item);
                    }
                }
            }
            _ => {}
        }
    }
}

#[atomic_delegates]
impl CallGraph {
    pub(crate) fn reachable(&self, roots: &[String], targets: &BTreeSet<String>) -> bool {}
    pub(crate) fn record_call(&mut self, name: &str, block: &syn::Block) {}
}

#[cfg(test)]
mod tests {
    use super::CallGraph;
    use lele_lint::Project;
    use std::path::PathBuf;

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("a.rs"),
            syn::parse_str("pub fn build() { setup(); } pub fn setup() {}").unwrap(),
        );
        let graph = CallGraph::from(&project);
        assert!(graph.reaches_production("setup"));
        assert!(!graph.reaches_production("unrelated"));
    }

    #[test]
    fn test_usage_chained_registrations_are_all_roots() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("a.rs"),
            syn::parse_str(
                "pub fn wire(app: &mut App) {
                     app.add_systems(Startup, spawn_hud).add_systems(Update, tick_hud);
                 }
                 fn spawn_hud() {}
                 fn tick_hud() {}",
            )
            .unwrap(),
        );
        let graph = CallGraph::from(&project);
        assert!(graph.reaches_production("spawn_hud"));
        assert!(graph.reaches_production("tick_hud"));
    }

    #[test]
    fn test_usage_observer_is_a_root() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("a.rs"),
            syn::parse_str(
                "pub fn wire(app: &mut App) { app.add_observer(spawn_on_ready); }
                 fn spawn_on_ready() {}",
            )
            .unwrap(),
        );
        let graph = CallGraph::from(&project);
        assert!(graph.reaches_production("spawn_on_ready"));
    }
}
