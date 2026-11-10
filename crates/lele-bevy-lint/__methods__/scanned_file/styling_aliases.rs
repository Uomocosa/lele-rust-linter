use std::collections::HashSet;

use syn::visit::Visit;

use crate::scan;

pub fn styling_aliases(file: &syn::File) -> HashSet<String> {
    let mut names = HashSet::new();
    let empty = HashSet::new();
    for item in &file.items {
        let syn::Item::Type(alias) = item else {
            continue;
        };
        let mut detector = scan::StylingDetector {
            aliases: &empty,
            found: false,
        };
        detector.visit_type(&alias.ty);
        if detector.found {
            names.insert(alias.ident.to_string());
        }
    }
    names
}

#[cfg(test)]
mod tests {
    use super::styling_aliases;

    #[test]
    fn test_usage() {
        let file: syn::File =
            syn::parse_str("type Parts<'a> = (Entity, &'a mut TextColor);").unwrap();
        let names = styling_aliases(&file);
        assert!(names.contains("Parts"));
    }
}
