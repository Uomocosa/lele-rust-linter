use crate::rules;
use derive_more::{Deref, DerefMut};
use syn::spanned::Spanned;
use syn::visit::Visit;

#[derive(Deref, DerefMut)]
pub(crate) struct CratePathVisitor<'a>(pub(crate) &'a mut Vec<rules::no_crate_paths::CratePathHit>);

#[rustfmt::skip]
impl<'ast> Visit<'ast> for CratePathVisitor<'_> {
    fn visit_path(&mut self, path: &'ast syn::Path) {
        if path
            .segments
            .first()
            .is_some_and(|seg| seg.ident == "crate")
        {
            let path_str = path
                .segments
                .iter()
                .map(|seg| seg.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");
            self.push(rules::no_crate_paths::CratePathHit {
                line: path.span().start().line,
                path: path_str,
            });
        }
        syn::visit::visit_path(self, path);
    }

    fn visit_visibility(&mut self, visibility: &'ast syn::Visibility) {
        match visibility {
            syn::Visibility::Restricted(_) => {}
            _ => syn::visit::visit_visibility(self, visibility),
        }
    }
}

// no test_usage necessary
