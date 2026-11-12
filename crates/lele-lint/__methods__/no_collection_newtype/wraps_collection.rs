use crate::rules;

pub fn wraps_collection(struct_def: &syn::ItemStruct) -> bool {
    let syn::Fields::Unnamed(fields) = &struct_def.fields else {
        return false;
    };
    fields.unnamed.len() == 1
        && fields.unnamed.first().is_some_and(|field| {
            rules::no_collection_newtype::NoCollectionNewtype::collection_name(&field.ty).is_some()
        })
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let wrapper: syn::ItemStruct = syn::parse_quote!(
            pub struct Deck(Vec<Card>);
        );
        let named: syn::ItemStruct = syn::parse_quote!(
            pub struct Deck {
                cards: Vec<Card>,
            }
        );
        assert!(rules::no_collection_newtype::NoCollectionNewtype::wraps_collection(&wrapper));
        assert!(!rules::no_collection_newtype::NoCollectionNewtype::wraps_collection(&named));
    }
}
