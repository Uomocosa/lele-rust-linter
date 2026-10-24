use crate::rules;
use std::collections::HashMap;

pub fn collect_vis_fields(file: &syn::File) -> HashMap<String, String> {
    let mut fields = HashMap::new();
    for item in &file.items {
        if let syn::Item::Struct(s) = item {
            for field in &s.fields {
                if let Some(vis_str) =
                    rules::no_trivial_accessors::NoTrivialAccessors::vis_to_string(&field.vis)
                {
                    if let Some(ident) = &field.ident {
                        fields.insert(ident.to_string(), vis_str);
                    }
                }
            }
        }
    }
    fields
}

// no test_usage necessary
