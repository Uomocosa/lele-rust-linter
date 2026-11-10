use syn::visit::Visit;

use crate::scan;

pub fn record_call(graph: &mut scan::CallGraph, name: &str, block: &syn::Block) {
    let mut calls = Vec::new();
    let mut systems_found = Vec::new();
    {
        let mut collector = scan::CallCollector {
            calls: &mut calls,
            systems: &mut systems_found,
        };
        collector.visit_block(block);
    }
    graph
        .callees
        .entry(name.to_string())
        .or_default()
        .extend(calls);
    graph.systems.extend(systems_found);
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use lele_lint::Project;

    use super::record_call;
    use crate::scan::CallGraph;

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("a.rs"),
            syn::parse_str("pub fn build() { setup(); } pub fn setup() {}").unwrap(),
        );
        let mut graph = CallGraph::from(&project);
        let block: syn::Block = syn::parse_quote! {{
            extra();
        }};
        record_call(&mut graph, "build", &block);
        assert!(graph.reaches_production("extra"));
    }
}
