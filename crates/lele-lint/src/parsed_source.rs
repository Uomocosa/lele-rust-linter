use std::path::Path;

use atomic_delegate_macros::atomic_delegates;

use crate::Origin;
use crate::RuleViolation;

pub struct ParsedSource<'a> {
    pub origin: Origin,
    pub relative_path: &'a Path,
    pub file: &'a syn::File,
}

#[atomic_delegates]
impl ParsedSource<'_> {
    pub fn scan_struct_items(
        &self,
        file_path: &Path,
        check: impl Fn(&syn::ItemStruct, &Path) -> Vec<RuleViolation>,
    ) -> Vec<RuleViolation> {
    }
    pub fn has_test_usage(&self) -> bool {}
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::Origin;
    use crate::ParsedSource;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_str(
            "pub struct A(u8);\n#[cfg(test)] mod tests { #[test] fn test_usage() {} }",
        )
        .unwrap();
        let source = ParsedSource {
            origin: Origin::Src,
            relative_path: Path::new("a.rs"),
            file: &file,
        };
        assert!(source.has_test_usage());
        let names = source.scan_struct_items(Path::new("a.rs"), |item, _| {
            assert_eq!(item.ident, "A");
            Vec::new()
        });
        assert!(names.is_empty());
        let missing: syn::File = syn::parse_str("pub fn compute() {}").unwrap();
        let without = ParsedSource {
            origin: Origin::Src,
            relative_path: Path::new("b.rs"),
            file: &missing,
        };
        assert!(!without.has_test_usage());
    }
}
