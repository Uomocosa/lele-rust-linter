use std::path::PathBuf;

use lele_lint::Project;

use crate::scan;

pub(crate) fn call_graph(source: &str) -> (syn::File, scan::CallGraph) {
    let file: syn::File = syn::parse_str(source).unwrap();
    let mut project = Project::default();
    project
        .parsed_files
        .insert(PathBuf::from("ui/gate.rs"), file.clone());
    let graph = scan::CallGraph::from(&project);
    (file, graph)
}

#[cfg(test)]
mod tests {
    use super::call_graph;

    #[test]
    fn test_usage() {
        let (file, graph) = call_graph("pub fn setup() { helper(); } pub fn helper() {}");
        assert_eq!(file.items.len(), 2);
        assert!(graph.reaches_production("helper"));
    }
}
