use std::collections::BTreeSet;

use crate::scan;

#[must_use]
pub fn reachable_visual(
    file: &scan::ScannedFile,
    graph: &scan::CallGraph,
    extra: &BTreeSet<String>,
) -> Option<scan::FoundVisual> {
    if !file.file_reaches_production(graph) {
        return None;
    }
    file.prod_visuals(extra)
        .into_iter()
        .find(|found| graph.reaches_production(&found.owner))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::reachable_visual;
    use crate::scan;
    use lele_lint::Project;

    #[test]
    fn test_usage() {
        let graph = scan::CallGraph::from(&Project::default());
        let file = syn::parse_str(
            "pub struct Spawner;
             pub fn setup(spawner: &mut Spawner) { spawner.spawn((Node, Text::new(\"x\"))); }",
        )
        .unwrap();
        let visual = reachable_visual(&scan::ScannedFile(&file), &graph, &BTreeSet::new()).unwrap();
        assert_eq!(visual.visual, "Node");
        assert_eq!(visual.owner, "setup");

        let no_ui = syn::parse_str("pub fn setup(c: &mut S) { c.spawn((Camera2d,)); }").unwrap();
        assert!(reachable_visual(&scan::ScannedFile(&no_ui), &graph, &BTreeSet::new()).is_none());
    }
}
