pub fn derive_names(attrs: &[syn::Attribute]) -> Vec<String> {
    let mut names = Vec::new();
    for attr in attrs {
        if !attr.path().is_ident("derive") {
            continue;
        }
        let parsed = attr.parse_args_with(
            syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
        );
        if let Ok(paths) = parsed {
            for path in paths {
                if let Some(segment) = path.segments.last() {
                    names.push(segment.ident.to_string());
                }
            }
        }
    }
    names
}

// no test_usage necessary
