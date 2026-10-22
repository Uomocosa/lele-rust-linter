use lele_snake_case::ToLeleSnakeCase;

use crate::common;

pub fn primary_type_name(file: &common::SourceFile, file_stem: &str) -> Option<String> {
    let mut declared = file.items.iter().filter_map(|item| match item {
        syn::Item::Struct(struct_item) => Some(struct_item.ident.to_string()),
        syn::Item::Enum(enum_item) => Some(enum_item.ident.to_string()),
        _ => None,
    });
    declared.find(|name| name.to_snake_case() == file_stem)
}

#[cfg(test)]
mod tests {
    use super::primary_type_name;
    use crate::common;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_str("pub struct AtomicFile;").unwrap();
        assert_eq!(
            primary_type_name(&common::SourceFile(&file), "atomic_file"),
            Some("AtomicFile".to_string())
        );
        assert_eq!(primary_type_name(&common::SourceFile(&file), "other"), None);
    }
}
