use atomic_delegate_macros::atomic_delegate;
use derive_more::Deref;
use derive_more::DerefMut;
use syn::visit::Visit;

use crate::scan::path_collector::PathCollector;

#[derive(Deref, DerefMut)]
pub(crate) struct FullPaths<'a, 'b>(pub(crate) &'a mut PathCollector<'b>);

impl<'ast> Visit<'ast> for FullPaths<'_, '_> {
    #[atomic_delegate(FullPaths, Visit)]
    fn visit_path(&mut self, node: &'ast syn::Path) {}
}
