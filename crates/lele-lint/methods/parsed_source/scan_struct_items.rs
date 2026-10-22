use crate::ParsedSource;
use std::path::Path;

use crate::RuleViolation;

pub fn scan_struct_items(
    source: &ParsedSource,
    file_path: &Path,
    check: impl Fn(&syn::ItemStruct, &Path) -> Vec<RuleViolation>,
) -> Vec<RuleViolation> {
    fn recursion(
        items: &[syn::Item],
        file_path: &Path,
        check: &impl Fn(&syn::ItemStruct, &Path) -> Vec<RuleViolation>,
        out: &mut Vec<RuleViolation>,
    ) {
        for item in items {
            match item {
                syn::Item::Struct(struct_def) => out.extend(check(struct_def, file_path)),
                syn::Item::Mod(module) => {
                    if let Some((_, inner)) = &module.content {
                        recursion(inner, file_path, check, out);
                    }
                }
                _ => {}
            }
        }
    }

    let mut out = Vec::new();
    recursion(&source.file.items, file_path, &check, &mut out);
    out
}

#[cfg(test)]
mod tests {
    use crate::Origin;
    use crate::ParsedSource;
    use std::path::Path;

    use crate::RuleViolation;

    use super::scan_struct_items;

    #[test]
    fn test_usage() {
        let file = syn::parse_str::<syn::File>("mod inner { pub struct A(pub u64); }").unwrap();
        let source = ParsedSource {
            origin: Origin::Src,
            relative_path: Path::new("x.rs"),
            file: &file,
        };
        let found = scan_struct_items(&source, Path::new("x.rs"), |_, _| {
            vec![RuleViolation::from_file(
                std::path::PathBuf::from("x.rs"),
                "LELE018",
                String::new(),
            )]
        });
        assert_eq!(found.len(), 1);
    }
}
