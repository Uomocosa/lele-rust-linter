use crate::rules;
use crate::ItemMarker;
use crate::RuleViolation;
use std::path::Path;

pub fn check_struct(struct_def: &syn::ItemStruct, file_path: &Path) -> Vec<RuleViolation> {
    if struct_def.fields.len() != 1 {
        return Vec::new();
    }
    let syn::Fields::Unnamed(fields) = &struct_def.fields else {
        return Vec::new();
    };
    let Some(field) = fields.unnamed.first() else {
        return Vec::new();
    };
    if struct_def
        .attrs
        .iter()
        .any(|attr| ItemMarker(attr).derives_data_shape())
    {
        return Vec::new();
    }
    let Some(collection) =
        rules::no_collection_newtype::NoCollectionNewtype::collection_name(&field.ty)
    else {
        return Vec::new();
    };
    if rules::no_collection_newtype::NoCollectionNewtype::has_unsigned_element(&field.ty) {
        return Vec::new();
    }
    let name = struct_def.ident.to_string();
    let singular = rules::no_collection_newtype::NoCollectionNewtype::singular_name(&name);
    vec![RuleViolation::from_file(file_path.to_path_buf(), rules::no_collection_newtype::NoCollectionNewtype::CODE, format!(
            "{name} wraps {collection}<T>; define singular {singular}(T) with Deref and use Vec<{singular}> at call sites"
        ))]
}

// no test_usage necessary
