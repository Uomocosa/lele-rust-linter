pub fn source_field_name(fields: &syn::Fields) -> Option<String> {
    fields.iter().enumerate().find_map(|(index, field)| {
        let is_marked = field
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("from") || attr.path().is_ident("source"));
        let name = field
            .ident
            .as_ref()
            .map_or_else(|| index.to_string(), ToString::to_string);
        (is_marked || name == "source").then_some(name)
    })
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let marked: syn::Variant = syn::parse_quote! { Io { #[from] inner: std::io::Error } };
        assert_eq!(
            rules::error_message_omits_source::ErrorMessageOmitsSource::source_field_name(
                &marked.fields
            )
            .as_deref(),
            Some("inner")
        );
        let plain: syn::Variant = syn::parse_quote! { Parse(String) };
        assert!(
            rules::error_message_omits_source::ErrorMessageOmitsSource::source_field_name(
                &plain.fields
            )
            .is_none()
        );
    }
}
