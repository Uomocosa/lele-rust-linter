use crate::ItemMarker;
use crate::ParsedSource;

pub fn has_test_usage(parsed_source: &ParsedSource) -> bool {
    parsed_source.file.items.iter().any(|item| {
        let syn::Item::Mod(module) = item else {
            return false;
        };
        module
            .attrs
            .iter()
            .any(|attr| ItemMarker(attr).is_cfg_test())
            && module.content.as_ref().is_some_and(|(_, items)| {
                items.iter().any(
                    |inner| matches!(inner, syn::Item::Fn(func) if func.sig.ident == "test_usage"),
                )
            })
    })
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::has_test_usage;
    use crate::Origin;
    use crate::ParsedSource;

    #[test]
    fn test_usage() {
        let with: syn::File =
            syn::parse_str("#[cfg(test)] mod tests { #[test] fn test_usage() {} }").unwrap();
        let source = ParsedSource {
            origin: Origin::Src,
            relative_path: Path::new("a.rs"),
            file: &with,
        };
        assert!(has_test_usage(&source));
        let without: syn::File = syn::parse_str("pub fn compute() {}").unwrap();
        let source = ParsedSource {
            origin: Origin::Src,
            relative_path: Path::new("b.rs"),
            file: &without,
        };
        assert!(!has_test_usage(&source));
    }
}
