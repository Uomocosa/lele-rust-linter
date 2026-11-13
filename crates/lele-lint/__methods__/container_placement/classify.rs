use crate::rules;

pub fn classify(item: &syn::Item) -> Option<rules::container_placement::Role> {
    match item {
        syn::Item::Struct(s) => {
            let derives = rules::container_placement::ContainerPlacement::derive_names(&s.attrs);
            if rules::container_placement::ContainerPlacement::is_excluded(&derives) {
                return None;
            }
            if let Some(role) = rules::container_placement::ContainerPlacement::ecs_role(&derives) {
                return Some(role);
            }
            if matches!(s.fields, syn::Fields::Unnamed(ref f) if f.unnamed.len() == 1) {
                Some(rules::container_placement::Role::Newtype)
            } else {
                Some(rules::container_placement::Role::Struct)
            }
        }
        syn::Item::Enum(e) => {
            let derives = rules::container_placement::ContainerPlacement::derive_names(&e.attrs);
            if rules::container_placement::ContainerPlacement::is_excluded(&derives) {
                return None;
            }
            rules::container_placement::ContainerPlacement::ecs_role(&derives)
                .or(Some(rules::container_placement::Role::Enum))
        }
        syn::Item::Type(_) => Some(rules::container_placement::Role::TypeAlias),
        syn::Item::Const(_) | syn::Item::Static(_) => Some(rules::container_placement::Role::Const),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let newtype: syn::Item = syn::parse_quote! { pub struct Meters(pub f64); };
        assert!(matches!(
            rules::container_placement::ContainerPlacement::classify(&newtype),
            Some(rules::container_placement::Role::Newtype)
        ));
        let record: syn::Item = syn::parse_quote! { pub struct Stock { count: u8 } };
        assert!(matches!(
            rules::container_placement::ContainerPlacement::classify(&record),
            Some(rules::container_placement::Role::Struct)
        ));
        let excluded: syn::Item = syn::parse_quote! { #[derive(Error)] pub enum Failure { Bad } };
        assert!(rules::container_placement::ContainerPlacement::classify(&excluded).is_none());
        let resource: syn::Item = syn::parse_quote! { #[derive(Resource)] pub struct Score(u32); };
        assert!(matches!(
            rules::container_placement::ContainerPlacement::classify(&resource),
            Some(rules::container_placement::Role::Resource)
        ));
    }
}
