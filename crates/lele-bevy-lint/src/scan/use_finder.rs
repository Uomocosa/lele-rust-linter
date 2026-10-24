use derive_more::Deref;
use derive_more::DerefMut;
use syn::visit::Visit;

use crate::scan;

#[derive(Deref, DerefMut)]
pub(crate) struct UseFinder<'a>(pub(crate) &'a mut bool);

#[rustfmt::skip]
impl<'ast> Visit<'ast> for UseFinder<'_> {
    fn visit_item_use(&mut self, node: &'ast syn::ItemUse) {
        if scan::ScannedFile::imports_run(&node.tree, false) {
            ***self = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::UseFinder;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_str("use lele_bevy_preview::run;").unwrap();
        let mut found = false;
        let mut visitor = UseFinder(&mut found);
        visitor.visit_file(&file);
        assert!(found);
    }
}
