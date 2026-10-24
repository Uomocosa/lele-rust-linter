pub fn advice(sig: &syn::Signature, self_name: &str, shape: &str) -> Option<String> {
    let [syn::FnArg::Typed(argument)] = sig.inputs.iter().collect::<Vec<_>>()[..] else {
        return None;
    };
    let source = quote::ToTokens::to_token_stream(&*argument.ty).to_string();
    let name = &sig.ident;
    match shape {
        "Self" => Some(format!(
            "`{self_name}::{name}` converts `{source}` into `{self_name}` — write it as `impl From<{source}> for {self_name}` and call `{self_name}::from(..)` / `.into()`"
        )),
        "Result" => Some(format!(
            "`{self_name}::{name}` is a fallible conversion from `{source}` — write it as `impl TryFrom<{source}> for {self_name}` and call `{self_name}::try_from(..)` / `.try_into()`"
        )),
        _ => None,
    }
}

// no test_usage necessary
