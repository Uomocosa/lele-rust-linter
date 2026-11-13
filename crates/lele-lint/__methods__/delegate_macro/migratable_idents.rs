use crate::FnBody;
use crate::ItemMarker;

pub fn migratable_idents(impl_block: &syn::ItemImpl) -> Vec<&syn::Ident> {
    impl_block
        .items
        .iter()
        .filter_map(|item| match item {
            syn::ImplItem::Fn(method)
                if !method
                    .attrs
                    .iter()
                    .any(|attr| ItemMarker(attr).is_atomic_delegate())
                    && (method.block.stmts.is_empty()
                        || FnBody(&method.block).is_delegate_call()
                        || FnBody(&method.block).is_methods_dispatch()) =>
            {
                Some(&method.sig.ident)
            }
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let block: syn::ItemImpl = syn::parse_quote! { impl Thing { fn shell(&self) -> u8 {} fn real(&self) -> u8 { self.a + 1 } } };
        let idents = rules::delegate_macro::DelegateMacro::migratable_idents(&block);
        assert_eq!(idents.len(), 1);
        assert_eq!(idents[0], "shell");
    }
}
