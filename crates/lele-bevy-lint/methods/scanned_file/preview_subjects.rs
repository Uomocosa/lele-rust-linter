use std::collections::BTreeSet;

use crate::scan;

#[must_use]
pub fn preview_subjects(
    file: &scan::ScannedFile,
    graph: &scan::CallGraph,
    extra: &BTreeSet<String>,
) -> Vec<String> {
    let mut subjects: Vec<String> = file
        .prod_visuals(extra)
        .into_iter()
        .filter(|found| !found.owner.is_empty() && graph.reaches_production(&found.owner))
        .map(|found| found.owner)
        .chain(file.plugin_types())
        .collect();
    subjects.sort();
    subjects.dedup();
    subjects
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::preview_subjects;
    use crate::scan;
    use lele_lint::Project;

    #[test]
    fn test_usage() {
        let graph = scan::CallGraph::from(&Project::default());
        let file = syn::parse_str(
            "pub fn setup(s: &mut S) { s.spawn((Node,)); }
             pub struct Hud;
             impl Plugin for Hud { fn build(&self, app: &mut A) { setup(app); } }",
        )
        .unwrap();
        assert_eq!(
            preview_subjects(&scan::ScannedFile(&file), &graph, &BTreeSet::new()),
            vec!["Hud".to_string(), "setup".to_string()]
        );
    }
}
