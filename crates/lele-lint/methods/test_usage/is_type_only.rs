use crate::rules;

pub fn is_type_only(file: &syn::File) -> bool {
    let has_struct_or_enum = file
        .items
        .iter()
        .any(|item| matches!(item, syn::Item::Struct(_) | syn::Item::Enum(_)));
    if !has_struct_or_enum {
        return false;
    }

    let has_non_default_impl = file.items.iter().any(|item| {
        if let syn::Item::Impl(impl_block) = item {
            if impl_block.trait_.is_some() {
                return false;
            }
            if !rules::test_usage::TestUsage::is_default_only_impl(impl_block) {
                return true;
            }
        }
        false
    });

    !has_non_default_impl
}

// no test_usage necessary
