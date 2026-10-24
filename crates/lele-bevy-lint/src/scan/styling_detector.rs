use std::collections::HashSet;

use syn::visit::Visit;

use crate::scan;

pub(crate) struct StylingDetector<'a> {
    pub(crate) aliases: &'a HashSet<String>,
    pub(crate) found: bool,
}

#[rustfmt::skip]
impl<'ast> Visit<'ast> for StylingDetector<'_> {
    fn visit_type_reference(&mut self, node: &'ast syn::TypeReference) {
        if node.mutability.is_some() && scan::ScannedFile::is_styling(&node.elem) {
            self.found = true;
        }
        syn::visit::visit_type_reference(self, node);
    }
    fn visit_path(&mut self, node: &'ast syn::Path) {
        if node.segments.iter().any(|segment| self.aliases.contains(&segment.ident.to_string())) {
            self.found = true;
        }
        syn::visit::visit_path(self, node);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use syn::visit::Visit;

    use super::StylingDetector;

    #[test]
    fn test_usage() {
        let ty: syn::Type = syn::parse_str("&mut BackgroundColor").unwrap();
        let mut detector = StylingDetector {
            aliases: &HashSet::new(),
            found: false,
        };
        detector.visit_type(&ty);
        assert!(detector.found);
    }
}
