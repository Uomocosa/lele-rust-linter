use atomic_delegate_macros::atomic_delegates;
use std::collections::HashSet;

use syn::visit::Visit;

pub(crate) struct StylingDetector<'a> {
    pub(crate) aliases: &'a HashSet<String>,
    pub(crate) found: bool,
}

#[atomic_delegates]
impl<'ast> Visit<'ast> for StylingDetector<'_> {
    fn visit_type_reference(&mut self, node: &'ast syn::TypeReference) {}
    fn visit_path(&mut self, node: &'ast syn::Path) {}
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
