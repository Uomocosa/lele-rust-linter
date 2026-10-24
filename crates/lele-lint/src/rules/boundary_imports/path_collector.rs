use crate::rules;
use std::collections::HashMap;
use syn::spanned::Spanned;
use syn::visit::Visit;

pub(crate) struct PathCollector {
    pub(crate) aliases: HashMap<String, String>,
    pub(crate) hits: Vec<rules::boundary_imports::PathHit>,
}

impl<'ast> Visit<'ast> for PathCollector {
    fn visit_path(&mut self, path: &'ast syn::Path) {
        let raw = path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>()
            .join("::");
        self.hits.push(rules::boundary_imports::PathHit {
            path: rules::boundary_imports::BoundaryImports::expand(&raw, &self.aliases),
            line: path.span().start().line,
            glob: false,
        });
        syn::visit::visit_path(self, path);
    }
}

// no test_usage necessary
