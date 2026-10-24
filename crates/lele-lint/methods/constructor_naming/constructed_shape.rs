use crate::rules;

pub fn constructed_shape(output: &syn::ReturnType, self_name: &str) -> Option<&'static str> {
    let syn::ReturnType::Type(_, ty) = output else {
        return None;
    };
    if rules::constructor_naming::ConstructorNaming::is_self(ty, self_name) {
        return Some("Self");
    }
    let syn::Type::Path(path) = &**ty else {
        return None;
    };
    let segment = path.path.segments.last()?;
    let wrapper = match segment.ident.to_string().as_str() {
        "Result" => "Result",
        "Option" => "Option",
        _ => return None,
    };
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };
    match args.args.first()? {
        syn::GenericArgument::Type(inner)
            if rules::constructor_naming::ConstructorNaming::is_self(inner, self_name) =>
        {
            Some(wrapper)
        }
        _ => None,
    }
}

// no test_usage necessary
