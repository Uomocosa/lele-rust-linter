use crate::ImplBlock;
use crate::ItemMarker;

pub fn has_behavior(file: &syn::File, type_name: &str) -> bool {
    file.items.iter().any(|item| {
        let syn::Item::Impl(impl_block) = item else {
            return false;
        };
        if impl_block
            .attrs
            .iter()
            .any(|attr| ItemMarker(attr).is_atomic_delegates())
        {
            return true;
        }
        ImplBlock(impl_block).self_type_last().as_deref() == Some(type_name)
    })
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let with_method: syn::File =
            syn::parse_quote! { pub struct Thing; impl Thing { fn run(&self) {} } };
        assert!(
            rules::container_placement::ContainerPlacement::has_behavior(&with_method, "Thing")
        );
        let bare: syn::File = syn::parse_quote! { pub struct Thing; };
        assert!(!rules::container_placement::ContainerPlacement::has_behavior(&bare, "Thing"));
    }
}
