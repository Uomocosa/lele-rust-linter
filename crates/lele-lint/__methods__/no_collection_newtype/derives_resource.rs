use crate::ItemMarker;

pub fn derives_resource(struct_def: &syn::ItemStruct) -> bool {
    let attrs = &struct_def.attrs;
    attrs
        .iter()
        .any(|attr| ItemMarker(attr).derives_any_of(&["Resource"]))
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let resource: syn::ItemStruct =
            syn::parse_quote! { #[derive(Resource)] pub struct PeerNames(pub Vec<String>); };
        assert!(rules::no_collection_newtype::NoCollectionNewtype::derives_resource(&resource));
        let pathed: syn::ItemStruct = syn::parse_quote! {
            #[derive(bevy::prelude::Resource)] pub struct PeerNames(pub Vec<String>);
        };
        assert!(rules::no_collection_newtype::NoCollectionNewtype::derives_resource(&pathed));
        let component: syn::ItemStruct =
            syn::parse_quote! { #[derive(Component)] pub struct Members(pub Vec<String>); };
        assert!(!rules::no_collection_newtype::NoCollectionNewtype::derives_resource(&component));
        let message: syn::ItemStruct =
            syn::parse_quote! { #[derive(Message)] pub struct ChatLog(pub Vec<String>); };
        assert!(!rules::no_collection_newtype::NoCollectionNewtype::derives_resource(&message));
        let plain: syn::ItemStruct = syn::parse_quote! { pub struct Tags(pub Vec<String>); };
        assert!(!rules::no_collection_newtype::NoCollectionNewtype::derives_resource(&plain));
    }
}
