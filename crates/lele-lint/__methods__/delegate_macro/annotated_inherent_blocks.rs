use crate::rules;
use crate::ImplBlock;
use crate::ImplItemGroup;
use crate::ItemMarker;

pub fn annotated_inherent_blocks(file: &syn::File, primary: &str, group: ImplItemGroup) -> usize {
    file.items
        .iter()
        .filter(|item| match item {
            syn::Item::Impl(block) => {
                block.trait_.is_none()
                    && !block
                        .attrs
                        .iter()
                        .any(|attr| ItemMarker(attr).is_cfg_test())
                    && block
                        .attrs
                        .iter()
                        .any(|attr| ItemMarker(attr).is_atomic_delegates())
                    && ImplBlock(block).self_type_last().as_deref() == Some(primary)
                    && rules::delegate_macro::DelegateMacro::shell_order_groups(block).as_slice()
                        == [group]
            }
            _ => false,
        })
        .count()
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::ImplItemGroup;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_quote! {
            #[atomic_delegates]
            impl Thing {
                pub fn run(&self) {}
            }
        };
        assert_eq!(
            rules::delegate_macro::DelegateMacro::annotated_inherent_blocks(
                &file,
                "Thing",
                ImplItemGroup::PubFn
            ),
            1
        );
        assert_eq!(
            rules::delegate_macro::DelegateMacro::annotated_inherent_blocks(
                &file,
                "Thing",
                ImplItemGroup::Constructor
            ),
            0
        );
    }
}
