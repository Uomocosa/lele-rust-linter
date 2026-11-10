use crate::rules;

pub fn collection_name(ty: &syn::Type) -> Option<&'static str> {
    if let syn::Type::Reference(ref_ty) = ty {
        if matches!(*ref_ty.elem, syn::Type::Slice(_)) {
            if ref_ty.mutability.is_some() {
                return Some("&mut [T]");
            }
            return Some("&[T]");
        }
        return None;
    }
    let syn::Type::Path(path) = ty else {
        return None;
    };
    let segment = path.path.segments.last()?;
    let name = segment.ident.to_string();
    match name.as_str() {
        "Vec" => Some("Vec"),
        "HashSet" => Some("HashSet"),
        "BTreeSet" => Some("BTreeSet"),
        "VecDeque" => Some("VecDeque"),
        "HashMap" => Some("HashMap"),
        "BTreeMap" => Some("BTreeMap"),
        "Box" => {
            if rules::no_collection_newtype::NoCollectionNewtype::boxed_slice_element(segment)
                .is_some()
            {
                Some("Box<[T]>")
            } else {
                None
            }
        }
        _ => None,
    }
}

// no test_usage necessary
