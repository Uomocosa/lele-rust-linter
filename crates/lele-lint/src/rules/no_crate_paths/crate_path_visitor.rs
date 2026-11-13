use crate::rules;
use atomic_delegate_macros::atomic_delegate;
use derive_more::{Deref, DerefMut};
use syn::visit::Visit;

#[derive(Deref, DerefMut)]
pub(crate) struct CratePathVisitor<'a>(pub(crate) &'a mut Vec<rules::no_crate_paths::CratePathHit>);

impl<'ast> Visit<'ast> for CratePathVisitor<'_> {
    #[atomic_delegate(CratePathVisitor, Visit)]
    fn visit_path(&mut self, path: &'ast syn::Path) {}
    fn visit_visibility(&mut self, visibility: &'ast syn::Visibility) {
        match visibility {
            syn::Visibility::Restricted(_) => {}
            _ => syn::visit::visit_visibility(self, visibility),
        }
    }
}

// no test_usage necessary
