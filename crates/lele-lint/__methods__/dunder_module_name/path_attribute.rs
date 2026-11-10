pub fn path_attribute(attrs: &[syn::Attribute]) -> Option<String> {
    let attr = attrs.iter().find(|attr| attr.path().is_ident("path"))?;
    let syn::Meta::NameValue(name_value) = &attr.meta else {
        return None;
    };
    let syn::Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Str(value),
        ..
    }) = &name_value.value
    else {
        return None;
    };
    Some(value.value())
}

// no test_usage necessary
