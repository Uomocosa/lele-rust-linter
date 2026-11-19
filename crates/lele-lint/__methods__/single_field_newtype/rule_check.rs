use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn rule_check(
    _self: &rules::single_field_newtype::SingleFieldNewtype,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for source in project.content_sources() {
        let file_path = project.absolute_path(source.origin, source.relative_path);
        violations.extend(source.scan_struct_items(
            &file_path,
            rules::single_field_newtype::SingleFieldNewtype::check_struct,
        ));
    }

    violations
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::testing;
    use crate::ItemMarker;
    use crate::Project;

    use super::rule_check;
    use crate::rules;

    #[test]
    fn test_usage() {
        let rule = rules::single_field_newtype::SingleFieldNewtype;
        let project = Project::default();
        assert!(rule_check(&rule, &project).is_empty());
    }

    #[test]
    fn test_usage_detects_deref_derive() {
        assert!(
            rules::single_field_newtype::SingleFieldNewtype::has_deref_derive(
                &testing::parsed_item_struct("#[derive(Deref)] pub struct X(pub u64);")
            )
        );
        assert!(
            rules::single_field_newtype::SingleFieldNewtype::has_deref_derive(
                &testing::parsed_item_struct(
                    "#[derive(Clone, Deref, DerefMut)] pub struct X(pub u64);"
                )
            )
        );
        assert!(
            !rules::single_field_newtype::SingleFieldNewtype::has_deref_derive(
                &testing::parsed_item_struct("#[derive(Clone)] pub struct X(pub u64);")
            )
        );
        assert!(
            !rules::single_field_newtype::SingleFieldNewtype::has_deref_derive(
                &testing::parsed_item_struct("pub struct X(pub u64);")
            )
        );
    }

    #[test]
    fn test_usage_single_field_named_is_rejected() {
        let s = testing::parsed_item_struct("pub struct X { pub value: u64 }");
        let violations =
            rules::single_field_newtype::SingleFieldNewtype::check_struct(&s, Path::new("x.rs"));
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("tuple newtype"));
    }

    #[test]
    fn test_usage_single_named_with_deref_passes() {
        let s = testing::parsed_item_struct(
            "#[derive(Debug, Clone, Deref)] pub struct X { pub value: Vec<String> }",
        );
        let violations =
            rules::single_field_newtype::SingleFieldNewtype::check_struct(&s, Path::new("x.rs"));
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_single_field_without_deref_is_rejected() {
        let s = testing::parsed_item_struct("pub struct X(pub u64);");
        let violations =
            rules::single_field_newtype::SingleFieldNewtype::check_struct(&s, Path::new("x.rs"));
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("#[derive(Deref)]"));
    }

    #[test]
    fn test_usage_single_field_with_deref_passes() {
        let s = testing::parsed_item_struct("#[derive(Deref)] pub struct X(pub u64);");
        let violations =
            rules::single_field_newtype::SingleFieldNewtype::check_struct(&s, Path::new("x.rs"));
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_serde_single_field_passes() {
        for code in [
            "#[derive(Deserialize)] struct Cfg { lele: String }",
            "#[derive(Serialize, Deserialize)] struct Cfg { lele: String }",
            "#[derive(serde::Deserialize)] struct Cfg { lele: String }",
            "#[derive(Parser)] struct Args { root: String }",
            "#[derive(Args)] struct Cmd { root: String }",
        ] {
            let s = testing::parsed_item_struct(code);
            assert!(s
                .attrs
                .iter()
                .any(|attr| ItemMarker(attr).derives_data_shape()));
            let violations = rules::single_field_newtype::SingleFieldNewtype::check_struct(
                &s,
                Path::new("x.rs"),
            );
            assert!(violations.is_empty());
        }
    }

    #[test]
    fn test_usage_non_shape_single_field_rejected() {
        let s = testing::parsed_item_struct("#[derive(Clone, Debug)] struct X { value: u64 }");
        assert!(!s
            .attrs
            .iter()
            .any(|attr| ItemMarker(attr).derives_data_shape()));
        let violations =
            rules::single_field_newtype::SingleFieldNewtype::check_struct(&s, Path::new("x.rs"));
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_usage_multi_field_tuple_is_rejected() {
        let s = testing::parsed_item_struct("pub struct X(pub String, pub u32);");
        let violations =
            rules::single_field_newtype::SingleFieldNewtype::check_struct(&s, Path::new("x.rs"));
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("named fields"));
    }
}

// no test_usage necessary
