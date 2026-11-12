pub fn advice(sig: &syn::Signature, self_name: &str, shape: &str) -> Option<String> {
    let [syn::FnArg::Typed(argument)] = sig.inputs.iter().collect::<Vec<_>>()[..] else {
        return None;
    };
    let source = quote::ToTokens::to_token_stream(&*argument.ty).to_string();
    let name = &sig.ident;
    let is_bare_primitive = match &*argument.ty {
        syn::Type::Path(path) => path.path.get_ident().is_some_and(|ident| {
            matches!(
                ident.to_string().as_str(),
                "i8" | "i16"
                    | "i32"
                    | "i64"
                    | "i128"
                    | "isize"
                    | "u8"
                    | "u16"
                    | "u32"
                    | "u64"
                    | "u128"
                    | "usize"
                    | "f32"
                    | "f64"
                    | "bool"
                    | "char"
                    | "String"
            )
        }),
        syn::Type::Reference(reference) => {
            matches!(&*reference.elem, syn::Type::Path(path) if path.path.is_ident("str"))
        }
        _ => false,
    };
    let hint = |conversion: &str| {
        if is_bare_primitive {
            format!(" — name the argument first: wrap it in a newtype in the domain's `__basic__/` (for example `Capacity(usize)`), then write `impl {conversion}<Capacity> for {self_name}`")
        } else {
            String::new()
        }
    };
    match shape {
        "Self" => Some(format!(
            "`{self_name}::{name}` converts `{source}` into `{self_name}` — write it as `impl From<{source}> for {self_name}` and call `{self_name}::from(..)` / `.into()`{}",
            hint("From")
        )),
        "Result" => Some(format!(
            "`{self_name}::{name}` is a fallible conversion from `{source}` — write it as `impl TryFrom<{source}> for {self_name}` and call `{self_name}::try_from(..)` / `.try_into()`{}",
            hint("TryFrom")
        )),
        _ => None,
    }
}

// no test_usage necessary
