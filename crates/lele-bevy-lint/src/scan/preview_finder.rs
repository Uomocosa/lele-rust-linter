use atomic_delegate_macros::atomic_delegates;
use syn::visit::Visit;

use crate::scan;

pub(crate) struct PreviewFinder<'a, 'ast> {
    pub(crate) previews: &'a mut Vec<scan::Preview>,
    pub(crate) is_run_imported: bool,
    pub(crate) module_fns: scan::ModuleFns<'ast>,
}

#[atomic_delegates]
impl<'ast> Visit<'ast> for PreviewFinder<'_, 'ast> {
    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {}
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {}
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
