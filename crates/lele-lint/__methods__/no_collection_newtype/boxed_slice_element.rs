pub fn boxed_slice_element(segment: &syn::PathSegment) -> Option<&syn::Type> {
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };
    for arg in &args.args {
        if let syn::GenericArgument::Type(syn::Type::Slice(slice)) = arg {
            return Some(&slice.elem);
        }
    }
    None
}

// no test_usage necessary
