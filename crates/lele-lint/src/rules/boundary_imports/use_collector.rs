use crate::rules;
use std::collections::HashMap;
use syn::spanned::Spanned;
use syn::visit::Visit;

#[derive(Default)]
pub(crate) struct UseCollector {
    pub(crate) aliases: HashMap<String, String>,
    pub(crate) hits: Vec<rules::boundary_imports::PathHit>,
}

impl<'ast> Visit<'ast> for UseCollector {
    fn visit_item_use(&mut self, node: &'ast syn::ItemUse) {
        let line = node.span().start().line;
        let mut prefix = Vec::new();
        rules::boundary_imports::BoundaryImports::walk_use_tree(
            &node.tree,
            &mut prefix,
            self,
            line,
        );
    }
}

// no test_usage necessary
