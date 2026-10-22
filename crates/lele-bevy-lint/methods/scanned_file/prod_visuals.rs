use std::collections::{BTreeSet, HashMap};

use derive_more::{Deref, DerefMut};
use syn::visit::Visit;

use crate::VISUAL_IDENTS;
use crate::scan;

const SPAWN_METHODS: [&str; 4] = ["spawn", "spawn_batch", "insert", "with_child"];

#[must_use]
pub fn prod_visuals(file: &scan::ScannedFile, extra: &BTreeSet<String>) -> Vec<scan::FoundVisual> {
    let module_fns = module_fns_of(&file.items);
    let mut found = Vec::new();
    let mut owner = String::new();
    {
        let mut scanner = UiScanner {
            found: &mut found,
            owner: &mut owner,
            module_fns: &module_fns,
            extra,
        };
        for item in &file.items {
            if scan::ScannedItem(item).is_test_module() {
                continue;
            }
            scanner.visit_item(item);
        }
    }
    found
}

// needed helper: the fns declared directly in the file, by name
fn module_fns_of(items: &[syn::Item]) -> HashMap<String, &syn::ItemFn> {
    items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Fn(func) => Some((func.sig.ident.to_string(), func)),
            _ => None,
        })
        .collect()
}

// needed helper: collects every non-test spawn bundle containing a visual ident
struct UiScanner<'a, 'b> {
    found: &'a mut Vec<scan::FoundVisual>,
    owner: &'a mut String,
    module_fns: &'a HashMap<String, &'b syn::ItemFn>,
    extra: &'a BTreeSet<String>,
}

impl<'ast> Visit<'ast> for UiScanner<'_, '_> {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        let outer = std::mem::take(self.owner);
        *self.owner = node.sig.ident.to_string();
        syn::visit::visit_item_fn(self, node);
        *self.owner = outer;
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        if SPAWN_METHODS.contains(&node.method.to_string().as_str()) {
            let mut collected = Vec::new();
            {
                let mut names = PathNames(&mut collected);
                for arg in &node.args {
                    names.visit_expr(arg);
                    if let Some(bundle) = called_sibling(arg, self.module_fns) {
                        names.visit_block(&bundle.block);
                    }
                }
            }
            if let Some(visual) = collected.iter().find(|name| {
                VISUAL_IDENTS.contains(&name.as_str()) || self.extra.contains(name.as_str())
            }) {
                self.found.push(scan::FoundVisual {
                    visual: visual.clone(),
                    line: node.method.span().start().line,
                    owner: self.owner.clone(),
                });
            }
        }
        syn::visit::visit_expr_method_call(self, node);
    }
}

// needed helper: the sibling fn a bare `name(..)` spawn argument calls, if any
fn called_sibling<'b>(
    arg: &syn::Expr,
    module_fns: &HashMap<String, &'b syn::ItemFn>,
) -> Option<&'b syn::ItemFn> {
    let syn::Expr::Call(call) = arg else {
        return None;
    };
    let syn::Expr::Path(path) = &*call.func else {
        return None;
    };
    if path.path.segments.len() != 1 {
        return None;
    }
    let name = path.path.segments.first()?.ident.to_string();
    module_fns.get(&name).copied()
}

// needed helper: every path ident inside a spawn argument, macro bodies included
#[derive(Deref, DerefMut)]
struct PathNames<'a>(&'a mut Vec<String>);

impl<'ast> Visit<'ast> for PathNames<'_> {
    fn visit_path(&mut self, node: &'ast syn::Path) {
        for segment in &node.segments {
            self.push(segment.ident.to_string());
        }
        syn::visit::visit_path(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        for expr in scan::MacroCall(node).exprs() {
            self.visit_expr(&expr);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::scan;
    use std::collections::BTreeSet;

    use super::prod_visuals;

    fn none() -> BTreeSet<String> {
        BTreeSet::new()
    }

    #[test]
    fn test_usage() {
        let file = syn::parse_str(
            "pub struct Spawner;
             pub fn setup(spawner: &mut Spawner) { spawner.spawn((Node, Text::new(\"x\"))); }
             #[cfg(test)]
             mod tests { fn test_usage() { spawner.spawn((Node,)); } }",
        )
        .unwrap();
        let found = prod_visuals(&scan::ScannedFile(&file), &none());
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].visual, "Node");
        assert_eq!(found[0].owner, "setup");
    }

    #[test]
    fn test_usage_camera_only_is_not_a_visual() {
        let file = syn::parse_str("pub fn setup(c: &mut S) { c.spawn((Camera2d,)); }").unwrap();
        assert_eq!(prod_visuals(&scan::ScannedFile(&file), &none()).len(), 0);
    }

    #[test]
    fn test_usage_visual_inside_children_macro_counts() {
        let file = syn::parse_str(
            "pub fn setup(s: &mut S) { s.spawn((Root, children![Text::new(\"x\")])); }",
        )
        .unwrap();
        let found = prod_visuals(&scan::ScannedFile(&file), &none());
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].visual, "Text");
    }

    #[test]
    fn test_usage_button_alone_is_a_visual() {
        let file = syn::parse_str("pub fn setup(s: &mut S) { s.spawn((Button,)); }").unwrap();
        assert_eq!(prod_visuals(&scan::ScannedFile(&file), &none()).len(), 1);
    }

    #[test]
    fn test_usage_insert_of_a_visual_counts() {
        let file =
            syn::parse_str("pub fn setup(s: &mut S) { s.entity(e).insert((Node,)); }").unwrap();
        let found = prod_visuals(&scan::ScannedFile(&file), &none());
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].visual, "Node");
    }

    #[test]
    fn test_usage_with_child_bundle_counts() {
        let file = syn::parse_str("pub fn setup(s: &mut S) { s.with_child((Node,)); }").unwrap();
        assert_eq!(prod_visuals(&scan::ScannedFile(&file), &none()).len(), 1);
    }

    #[test]
    fn test_usage_spawned_bundle_fn_counts() {
        let file = syn::parse_str(
            "fn row() -> impl Bundle { (Node::default(), Text::new(\"x\")) }
             pub fn setup(s: &mut S) { s.spawn(row()); }",
        )
        .unwrap();
        let found = prod_visuals(&scan::ScannedFile(&file), &none());
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].owner, "setup");
    }

    #[test]
    fn test_usage_require_component_counts() {
        let mut extra = BTreeSet::new();
        extra.insert(String::from("Cell"));
        let file = syn::parse_str("pub fn setup(s: &mut S) { s.spawn((Cell,)); }").unwrap();
        assert_eq!(prod_visuals(&scan::ScannedFile(&file), &extra).len(), 1);
    }
}
