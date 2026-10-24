use crate::rules;
use std::collections::HashSet;
use syn::visit::Visit;

pub(crate) struct StutterVisitor<'a> {
    pub(crate) hits: &'a mut Vec<rules::no_stuttered_path::StutterHit>,
    pub(crate) stems: &'a HashSet<String>,
}

impl<'ast> Visit<'ast> for StutterVisitor<'_> {
    fn visit_path(&mut self, path: &'ast syn::Path) {
        rules::no_stuttered_path::NoStutteredPath::collect_stutter(path, self.stems, self.hits);
        syn::visit::visit_path(self, path);
    }
}

// no test_usage necessary
