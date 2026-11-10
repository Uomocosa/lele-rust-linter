pub fn is_plain_conversion(call: &syn::ExprMethodCall) -> bool {
    const CONVERSIONS: [&str; 7] = [
        "clone", "to_owned", "to_vec", "as_str", "as_slice", "as_ref", "as_deref",
    ];
    call.args.is_empty() && CONVERSIONS.iter().any(|name| call.method == name)
}

// no test_usage necessary
