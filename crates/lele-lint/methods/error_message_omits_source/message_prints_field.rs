pub fn message_prints_field(message: &syn::Attribute, field: &str) -> bool {
    let syn::Meta::List(list) = &message.meta else {
        return false;
    };
    let compact: String = list.tokens.to_string().split_whitespace().collect();
    let argument = format!(".{field}");
    compact.contains(&format!("{{{field}}}"))
        || compact.contains(&format!("{{{field}:"))
        || compact
            .split(',')
            .any(|part| part.trim_end_matches(')') == argument)
}

// no test_usage necessary
