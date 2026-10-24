use crate::rules;
use crate::ItemMarker;
use crate::RuleViolation;
use std::path::Path;

pub fn check_struct(struct_def: &syn::ItemStruct, file_path: &Path) -> Vec<RuleViolation> {
    let name = struct_def.ident.to_string();
    let field_count = struct_def.fields.len();

    if field_count == 1 {
        if let syn::Fields::Named(_) = struct_def.fields {
            if struct_def
                .attrs
                .iter()
                .any(|attr| ItemMarker(attr).derives_data_shape())
            {
                return Vec::new();
            }
            if rules::single_field_newtype::SingleFieldNewtype::has_deref_derive(struct_def) {
                return Vec::new();
            }
            return vec![RuleViolation::from_file(file_path.to_path_buf(), rules::single_field_newtype::SingleFieldNewtype::CODE, format!("{name} has a single named field without Deref and must be a tuple newtype like `pub struct {name}(pub T)`; named single-field structs must derive Deref"))];
        }
        if !rules::single_field_newtype::SingleFieldNewtype::has_deref_derive(struct_def) {
            return vec![RuleViolation::from_file(
                file_path.to_path_buf(),
                rules::single_field_newtype::SingleFieldNewtype::CODE,
                format!("{name} is a single-field tuple newtype and must derive Deref"),
            )];
        }
    } else if field_count >= 2 {
        if let syn::Fields::Unnamed(_) = struct_def.fields {
            return vec![RuleViolation::from_file(
                file_path.to_path_buf(),
                rules::single_field_newtype::SingleFieldNewtype::CODE,
                format!(
                    "{name} has multiple fields and must use named fields like `{{ a: A, b: B }}`"
                ),
            )];
        }
    }
    Vec::new()
}

// no test_usage necessary
