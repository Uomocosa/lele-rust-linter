use std::collections::BTreeSet;
use std::collections::HashMap;

use syn::visit::Visit;

use crate::VISUAL_IDENTS;
use crate::scan;

pub(crate) struct UiScanner<'a, 'b> {
    pub(crate) found: &'a mut Vec<scan::FoundVisual>,
    pub(crate) owner: &'a mut String,
    pub(crate) module_fns: &'a HashMap<String, &'b syn::ItemFn>,
    pub(crate) extra: &'a BTreeSet<String>,
}

const SPAWN_METHODS: [&str; 4] = ["spawn", "spawn_batch", "insert", "with_child"];

#[rustfmt::skip]
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
                let mut names = scan::PathNames(&mut collected);
                for arg in &node.args {
                    names.visit_expr(arg);
                    if let Some(bundle) = scan::ScannedFile::called_sibling(arg, self.module_fns) {
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use syn::visit::Visit;

    use crate::scan;

    use super::UiScanner;

    #[test]
    fn test_usage() {
        let file: syn::File =
            syn::parse_str("pub fn setup(s: &mut S) { s.spawn((Node,)); }").unwrap();
        let module_fns = scan::ScannedFile::module_fns_of(&file.items);
        let mut found = Vec::new();
        let mut owner = String::new();
        let mut scanner = UiScanner {
            found: &mut found,
            owner: &mut owner,
            module_fns: &module_fns,
            extra: &BTreeSet::new(),
        };
        scanner.visit_file(&file);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].visual, "Node");
    }
}
