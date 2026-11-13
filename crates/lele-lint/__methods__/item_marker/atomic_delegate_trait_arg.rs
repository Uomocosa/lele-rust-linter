use crate::ItemMarker;

pub fn atomic_delegate_trait_arg(marker: ItemMarker) -> Option<String> {
    marker
        .parse_args_with(
            syn::punctuated::Punctuated::<syn::Ident, syn::Token![,]>::parse_terminated,
        )
        .ok()
        .and_then(|args| args.into_iter().nth(1))
        .map(|ident| ident.to_string())
}

#[cfg(test)]
mod tests {
    use super::atomic_delegate_trait_arg;
    use crate::ItemMarker;

    #[test]
    fn test_usage() {
        let bare: syn::ImplItemFn =
            syn::parse_quote! { #[atomic_delegate(Foo)] fn check(&self) {} };
        let bare_marker = bare.attrs.first().map(ItemMarker).unwrap();
        assert_eq!(atomic_delegate_trait_arg(bare_marker), None);
        let with_trait: syn::ImplItemFn =
            syn::parse_quote! { #[atomic_delegate(Foo, Rule)] fn check(&self) {} };
        let trait_marker = with_trait.attrs.first().map(ItemMarker).unwrap();
        assert_eq!(
            atomic_delegate_trait_arg(trait_marker).as_deref(),
            Some("Rule")
        );
    }
}
