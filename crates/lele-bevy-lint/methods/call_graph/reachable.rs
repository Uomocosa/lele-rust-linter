use std::collections::BTreeSet;
use std::collections::HashSet;

use crate::scan;

pub fn reachable(graph: &scan::CallGraph, roots: &[String], targets: &BTreeSet<String>) -> bool {
    let mut stack: Vec<String> = roots.to_vec();
    let mut seen = HashSet::new();
    while let Some(current) = stack.pop() {
        if !seen.insert(current.clone()) {
            continue;
        }
        if targets.contains(&current) {
            return true;
        }
        if let Some(next) = graph.callees.get(&current) {
            stack.extend(next.iter().cloned());
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::PathBuf;

    use lele_lint::Project;

    use super::reachable;
    use crate::scan::CallGraph;

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("a.rs"),
            syn::parse_str("pub fn build() { setup(); } pub fn setup() {}").unwrap(),
        );
        let graph = CallGraph::from(&project);
        let roots = vec!["build".to_string()];
        let targets = BTreeSet::from(["setup".to_string()]);
        assert!(reachable(&graph, &roots, &targets));
        let missing = BTreeSet::from(["absent".to_string()]);
        assert!(!reachable(&graph, &roots, &missing));
    }
}
