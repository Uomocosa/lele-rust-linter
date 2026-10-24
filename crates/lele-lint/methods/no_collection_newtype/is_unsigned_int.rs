pub fn is_unsigned_int(ty: &syn::Type) -> bool {
    let syn::Type::Path(path) = ty else {
        return false;
    };
    let Some(segment) = path.path.segments.last() else {
        return false;
    };
    if !matches!(
        segment.ident.to_string().as_str(),
        "u8" | "u16" | "u32" | "u64" | "u128" | "usize"
    ) {
        return false;
    }
    matches!(segment.arguments, syn::PathArguments::None)
}

// no test_usage necessary
