use std::collections::HashSet;

use syn::spanned::Spanned;
use syn::visit::Visit;

use crate::STYLING_COMPONENTS;
use crate::scan;

#[must_use]
pub fn restyled_systems(file: &syn::File, graph: &scan::CallGraph) -> Vec<scan::Restyle> {
    let aliases = styling_aliases(file);
    let mut found = Vec::new();
    for item in &file.items {
        if scan::is_test_module(item) {
            continue;
        }
        let syn::Item::Fn(func) = item else {
            continue;
        };
        let system = func.sig.ident.to_string();
        if !graph.reaches_production(&system) {
            continue;
        }
        if signature_restyles(&func.sig, &aliases) {
            found.push(scan::Restyle {
                system,
                line: func.sig.fn_token.span().start().line,
            });
        }
    }
    found
}

// needed helper: local `type` aliases whose definition mentions a mutable styling component
fn styling_aliases(file: &syn::File) -> HashSet<String> {
    let mut names = HashSet::new();
    let empty = HashSet::new();
    for item in &file.items {
        let syn::Item::Type(alias) = item else {
            continue;
        };
        let mut detector = StylingDetector {
            aliases: &empty,
            found: false,
        };
        detector.visit_type(&alias.ty);
        if detector.found {
            names.insert(alias.ident.to_string());
        }
    }
    names
}

// needed helper: does a system signature take `&mut` on a styling component, directly or via an alias?
fn signature_restyles(sig: &syn::Signature, aliases: &HashSet<String>) -> bool {
    sig.inputs.iter().any(|input| {
        let syn::FnArg::Typed(pat) = input else {
            return false;
        };
        let mut detector = StylingDetector {
            aliases,
            found: false,
        };
        detector.visit_type(&pat.ty);
        detector.found
    })
}

// needed helper: matches a `&mut` styling component, or a reference to a styling type alias
struct StylingDetector<'a> {
    aliases: &'a HashSet<String>,
    found: bool,
}

impl<'ast> Visit<'ast> for StylingDetector<'_> {
    fn visit_type_reference(&mut self, node: &'ast syn::TypeReference) {
        if node.mutability.is_some() && is_styling(&node.elem) {
            self.found = true;
        }
        syn::visit::visit_type_reference(self, node);
    }

    fn visit_path(&mut self, node: &'ast syn::Path) {
        if node
            .segments
            .iter()
            .any(|segment| self.aliases.contains(&segment.ident.to_string()))
        {
            self.found = true;
        }
        syn::visit::visit_path(self, node);
    }
}

// needed helper: is this type a bare styling component?
fn is_styling(ty: &syn::Type) -> bool {
    let syn::Type::Path(path) = ty else {
        return false;
    };
    path.path
        .segments
        .last()
        .is_some_and(|segment| STYLING_COMPONENTS.contains(&segment.ident.to_string().as_str()))
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
        let graph = scan::CallGraph::collect(&project);
        (file, graph)
    }

    #[test]
    fn test_usage() {
        let (file, graph) = graph_for(
            "pub fn recolor(row: Query<&mut BackgroundColor>) { let _ = row; }
             pub fn build(app: &mut App) { app.add_systems(Update, recolor); }
             pub fn spawner(s: &mut S) { s.spawn((Node,)); }",
        );
        let found = restyled_systems(&file, &graph);
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
        let found = restyled_systems(&file, &graph);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].system, "recolor");
    }
}
