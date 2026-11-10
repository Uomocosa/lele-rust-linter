use syn::spanned::Spanned;

use crate::scan;

#[must_use]
pub fn restyled_systems(file: &scan::ScannedFile, graph: &scan::CallGraph) -> Vec<scan::Restyle> {
    let aliases = scan::ScannedFile::styling_aliases(file);
    let mut found = Vec::new();
    for item in &file.items {
        if scan::ScannedItem(item).is_test_module() {
            continue;
        }
        let syn::Item::Fn(func) = item else {
            continue;
        };
        let system = func.sig.ident.to_string();
        if !graph.reaches_production(&system) {
            continue;
        }
        if scan::ScannedFile::signature_restyles(&func.sig, &aliases) {
            found.push(scan::Restyle {
                system,
                line: func.sig.fn_token.span().start().line,
            });
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::restyled_systems;
    use crate::scan;
    use lele_lint::Project;
    use std::path::PathBuf;

    fn graph_for(source: &str) -> (syn::File, scan::CallGraph) {
        let file: syn::File = syn::parse_str(source).unwrap();
        let mut project = Project::default();
        project
            .parsed_files
            .insert(PathBuf::from("ui/gate.rs"), file.clone());
        let graph = scan::CallGraph::from(&project);
        (file, graph)
    }

    #[test]
    fn test_usage() {
        let (file, graph) = graph_for(
            "pub fn recolor(row: Query<&mut BackgroundColor>) { let _ = row; }
             pub fn build(app: &mut App) { app.add_systems(Update, recolor); }
             pub fn spawner(s: &mut S) { s.spawn((Node,)); }",
        );
        let found = restyled_systems(&scan::ScannedFile(&file), &graph);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].system, "recolor");
    }

    #[test]
    fn test_usage_alias_is_resolved() {
        let (file, graph) = graph_for(
            "type Parts<'a> = (Entity, &'a mut TextColor);
             pub fn recolor(button: Single<Parts>) { let _ = button; }
             pub fn build(app: &mut App) { app.add_systems(Update, recolor); }",
        );
        let found = restyled_systems(&scan::ScannedFile(&file), &graph);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].system, "recolor");
    }
}
