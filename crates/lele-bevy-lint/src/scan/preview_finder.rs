use syn::spanned::Spanned;
use syn::visit::Visit;

use crate::scan;

pub(crate) struct PreviewFinder<'a, 'ast> {
    pub(crate) previews: &'a mut Vec<scan::Preview>,
    pub(crate) is_run_imported: bool,
    pub(crate) module_fns: scan::ModuleFns<'ast>,
}

#[rustfmt::skip]
impl<'ast> Visit<'ast> for PreviewFinder<'_, 'ast> {
    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        let Some((_, items)) = &node.content else {
            return;
        };
        let outer = std::mem::replace(&mut self.module_fns, scan::ScannedFile::module_fns_of(items));
        syn::visit::visit_item_mod(self, node);
        self.module_fns = outer;
    }
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        let name = node.sig.ident.to_string();
        if let Some(kind) = scan::ScannedFile::kind_of(&name) {
            self.previews.push(scan::Preview {
                name,
                kind,
                line: node.sig.fn_token.span().start().line,
                is_test: node.attrs.iter().any(|attr| attr.path().is_ident("test")),
                harness: scan::HarnessCall::from_fn_body(&node.block, self.is_run_imported),
                scene: scan::SceneUsage::from_fn_body(&node.block, &self.module_fns),
            });
        }
        syn::visit::visit_item_fn(self, node);
    }
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use crate::scan;

    use super::PreviewFinder;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_str("fn wheel_ui_png_preview() { let _ = 1; }").unwrap();
        let mut previews = Vec::new();
        let module_fns = scan::ScannedFile::module_fns_of(&file.items);
        let mut finder = PreviewFinder {
            previews: &mut previews,
            is_run_imported: false,
            module_fns,
        };
        finder.visit_file(&file);
        assert_eq!(previews.len(), 1);
    }
}
