pub fn names_str(idents: &[&syn::Ident]) -> String {
    idents
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let alpha: syn::Ident = syn::parse_quote! { alpha };
        let beta: syn::Ident = syn::parse_quote! { beta };
        assert_eq!(
            rules::atomic_delegates::AtomicDelegates::names_str(&[&alpha, &beta]),
            "alpha, beta"
        );
        assert_eq!(rules::atomic_delegates::AtomicDelegates::names_str(&[]), "");
    }
}
