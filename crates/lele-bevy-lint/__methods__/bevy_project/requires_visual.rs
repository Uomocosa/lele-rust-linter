use crate::VISUAL_IDENTS;

pub fn requires_visual(attrs: &[syn::Attribute]) -> bool {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("require"))
        .any(|attr| {
            attr.parse_args_with(
                syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
            )
            .is_ok_and(|paths| {
                paths.iter().any(|path| {
                    path.segments.last().is_some_and(|segment| {
                        VISUAL_IDENTS.contains(&segment.ident.to_string().as_str())
                    })
                })
            })
        })
}

#[cfg(test)]
mod tests {
    use super::requires_visual;

    #[test]
    fn test_usage() {
        let visual: syn::ItemStruct = syn::parse_quote! {
            #[require(Transform, Node)]
            struct Panel;
        };
        let plain: syn::ItemStruct = syn::parse_quote! {
            #[require(Transform)]
            struct Marker;
        };
        assert!(requires_visual(&visual.attrs));
        assert!(!requires_visual(&plain.attrs));
    }
}
