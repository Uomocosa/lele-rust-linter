use atomic_delegate_macros::atomic_delegate;
use derive_more::Deref;
use derive_more::DerefMut;
use syn::visit::Visit;

use crate::scan::use_collector::UseCollector;

#[derive(Deref, DerefMut)]
pub(crate) struct RegisteredNames<'a, 'b>(pub(crate) &'a mut UseCollector<'b>);

impl<'ast> Visit<'ast> for RegisteredNames<'_, '_> {
    #[atomic_delegate(RegisteredNames, Visit)]
    fn visit_path(&mut self, node: &'ast syn::Path) {}
}
