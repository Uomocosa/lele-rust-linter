use crate::rules;

pub fn contains_entity(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(path) => {
            if path
                .path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == "Entity")
            {
                return true;
            }
            path.path
                .segments
                .iter()
                .any(|segment| match &segment.arguments {
                    syn::PathArguments::AngleBracketed(args) => {
                        args.args.iter().any(|arg| match arg {
                            syn::GenericArgument::Type(inner) => {
                                rules::no_serialized_entity::NoSerializedEntity::contains_entity(
                                    inner,
                                )
                            }
                            _ => false,
                        })
                    }
                    _ => false,
                })
        }
        syn::Type::Reference(item) => {
            rules::no_serialized_entity::NoSerializedEntity::contains_entity(&item.elem)
        }
        syn::Type::Ptr(item) => {
            rules::no_serialized_entity::NoSerializedEntity::contains_entity(&item.elem)
        }
        syn::Type::Slice(item) => {
            rules::no_serialized_entity::NoSerializedEntity::contains_entity(&item.elem)
        }
        syn::Type::Array(item) => {
            rules::no_serialized_entity::NoSerializedEntity::contains_entity(&item.elem)
        }
        syn::Type::Tuple(item) => item
            .elems
            .iter()
            .any(rules::no_serialized_entity::NoSerializedEntity::contains_entity),
        syn::Type::Paren(item) => {
            rules::no_serialized_entity::NoSerializedEntity::contains_entity(&item.elem)
        }
        syn::Type::Group(item) => {
            rules::no_serialized_entity::NoSerializedEntity::contains_entity(&item.elem)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::contains_entity;

    #[test]
    fn test_usage() {
        let nested: syn::Type = syn::parse_quote! { Vec<Option<Entity>> };
        let tuple: syn::Type = syn::parse_quote! { (u32, Entity) };
        let plain: syn::Type = syn::parse_quote! { Vec<u32> };
        assert!(contains_entity(&nested));
        assert!(contains_entity(&tuple));
        assert!(!contains_entity(&plain));
    }
}
