pub fn is_builder(method: &syn::ImplItemFn) -> bool {
    let name = method.sig.ident.to_string();
    if !name.starts_with("with_") {
        return false;
    }
    let Some(receiver) = method.sig.receiver() else {
        return false;
    };
    if receiver.reference.is_some() {
        return false;
    }
    if method.sig.inputs.len() != 2 {
        return false;
    }
    match &method.sig.output {
        syn::ReturnType::Type(_, ty) => {
            let s = quote::quote!(#ty).to_string();
            s.contains("Self")
        }
        syn::ReturnType::Default => false,
    }
}

// no test_usage necessary
