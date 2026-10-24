use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check(
    _self: &rules::no_collection_newtype::NoCollectionNewtype,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for source in project.content_sources() {
        let file_path = project.absolute_path(source.origin, source.relative_path);
        violations.extend(source.scan_struct_items(
            &file_path,
            rules::no_collection_newtype::NoCollectionNewtype::check_struct,
        ));
    }

    violations
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use std::path::Path;

    use syn::ItemStruct;

    fn parse(code: &str) -> ItemStruct {
        syn::parse_str(code).unwrap()
    }

    #[test]
    fn test_usage() {
        let s = parse("pub struct ExampleNames(pub Vec<String>);");
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("Vec<ExampleName>"));
    }

    #[test]
    fn test_usage_singular_newtype_passes() {
        let s = parse("pub struct ExampleName(pub String);");
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_map_newtype_is_rejected() {
        let s = parse("pub struct Scores(pub HashMap<String, u32>);");
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_usage_multi_field_vec_passes() {
        let s = parse("pub struct Player { pub names: Vec<String> }");
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_serde_exempt() {
        let s = parse("#[derive(Deserialize)] pub struct Tags(pub Vec<String>);");
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_singular_names() {
        assert_eq!(
            rules::no_collection_newtype::NoCollectionNewtype::singular_name("ExampleNames"),
            "ExampleName"
        );
        assert_eq!(
            rules::no_collection_newtype::NoCollectionNewtype::singular_name("Tags"),
            "Tag"
        );
        assert_eq!(
            rules::no_collection_newtype::NoCollectionNewtype::singular_name("Entries"),
            "Entry"
        );
        assert_eq!(
            rules::no_collection_newtype::NoCollectionNewtype::singular_name("Score"),
            "ScoreItem"
        );
    }

    #[test]
    fn test_usage_borrowed_slice_rejected() {
        let s = parse("pub struct ExampleNames<'a>(pub &'a [String]);");
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_usage_mut_slice_rejected() {
        let s = parse("pub struct ExampleNames<'a>(pub &'a mut [String]);");
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_usage_boxed_slice_rejected() {
        let s = parse("pub struct ExampleNames(pub Box<[String]>);");
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_usage_unsigned_elements_exempt() {
        for code in [
            "pub struct Bytes(pub Vec<u8>);",
            "pub struct Counts(pub Vec<usize>);",
            "pub struct Words(pub Vec<u32>);",
        ] {
            let s = parse(code);
            let violations = rules::no_collection_newtype::NoCollectionNewtype::check_struct(
                &s,
                Path::new("x.rs"),
            );
            assert!(violations.is_empty());
        }
    }

    #[test]
    fn test_usage_unsigned_slice_exempt() {
        let s = parse("pub struct Bytes<'a>(pub &'a [u8]);");
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(&s, Path::new("x.rs"));
        assert!(violations.is_empty());
    }
}
