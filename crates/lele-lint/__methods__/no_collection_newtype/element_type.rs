use crate::rules;

pub fn element_type(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Reference(ref_ty) = ty {
        if let syn::Type::Slice(slice) = &*ref_ty.elem {
            return Some(&slice.elem);
        }
        return None;
    }
    let syn::Type::Path(path) = ty else {
        return None;
    };
    let segment = path.path.segments.last()?;
    let name = segment.ident.to_string();
    match name.as_str() {
        "Box" => rules::no_collection_newtype::NoCollectionNewtype::boxed_slice_element(segment),
        "HashMap" | "BTreeMap" => None,
        _ => rules::no_collection_newtype::NoCollectionNewtype::first_generic_type(segment),
    }
}

// no test_usage necessary
