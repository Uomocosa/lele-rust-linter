use atomic_delegate_macros::atomic_delegates;
use std::collections::BTreeSet;
use std::collections::HashMap;

use syn::visit::Visit;

use crate::scan;

pub(crate) struct UiScanner<'a, 'b> {
    pub(crate) found: &'a mut Vec<scan::FoundVisual>,
    pub(crate) owner: &'a mut String,
    pub(crate) module_fns: &'a HashMap<String, &'b syn::ItemFn>,
    pub(crate) extra: &'a BTreeSet<String>,
}

#[atomic_delegates]
impl<'ast> Visit<'ast> for UiScanner<'_, '_> {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {}
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {}
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
