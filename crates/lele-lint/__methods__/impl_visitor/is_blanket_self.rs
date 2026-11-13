pub fn is_blanket_self(item: &syn::ItemImpl) -> bool {
    matches!(&*item.self_ty, syn::Type::Path(type_path)
    if type_path.path.get_ident().is_some_and(|ident| {
        item.generics.type_params().any(|param| param.ident == *ident)
    }))
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let blanket: syn::ItemImpl = syn::parse_quote! { impl<T> Show for T {} };
        assert!(rules::trait_impl_co_location::ImplVisitor::is_blanket_self(
            &blanket
        ));
        let concrete: syn::ItemImpl = syn::parse_quote! { impl Show for Circle {} };
        assert!(!rules::trait_impl_co_location::ImplVisitor::is_blanket_self(&concrete));
    }
}
