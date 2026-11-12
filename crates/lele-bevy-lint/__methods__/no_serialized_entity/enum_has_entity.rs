use crate::rules;

pub fn enum_has_entity(item_enum: &syn::ItemEnum) -> bool {
    item_enum
        .variants
        .iter()
        .any(|variant| match &variant.fields {
            syn::Fields::Named(fields) => fields.named.iter().any(|field| {
                rules::no_serialized_entity::NoSerializedEntity::contains_entity(&field.ty)
            }),
            syn::Fields::Unnamed(fields) => fields.unnamed.iter().any(|field| {
                rules::no_serialized_entity::NoSerializedEntity::contains_entity(&field.ty)
            }),
            syn::Fields::Unit => false,
        })
}

#[cfg(test)]
mod tests {
    use super::enum_has_entity;

    #[test]
    fn test_usage() {
        let holds: syn::ItemEnum = syn::parse_quote! {
            enum Delta { Joined { owner: Entity }, Left }
        };
        let plain: syn::ItemEnum = syn::parse_quote! {
            enum Delta { Joined(u32), Left }
        };
        assert!(enum_has_entity(&holds));
        assert!(!enum_has_entity(&plain));
    }
}
