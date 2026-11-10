pub fn advice(sig: &syn::Signature, self_name: &str, shape: &str) -> Option<String> {
    if sig.inputs.len() < 2 {
        return None;
    }
    let prefix = if shape == "Self" {
        "from_"
    } else {
        "try_from_"
    };
    let name = sig.ident.to_string();
    let is_fallible_name = name.starts_with("try_from_");
    let is_named = match prefix {
        "from_" => name.starts_with("from_"),
        _ => is_fallible_name,
    };
    if is_named {
        return None;
    }
    let suggested = name
        .strip_prefix("try_from_")
        .or_else(|| name.strip_prefix("from_"))
        .unwrap_or(&name);
    Some(format!(
        "`{self_name}::{name}` builds `{self_name}` from {} arguments — rename it `{prefix}<what it is built from>` (e.g. `{prefix}{suggested}`); `from_*` returns `Self`, `try_from_*` returns `Result<Self, _>`/`Option<Self>`",
        sig.inputs.len()
    ))
}

// no test_usage necessary
