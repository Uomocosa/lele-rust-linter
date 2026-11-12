use crate::rules;
use crate::ImplBlock;

pub fn is_atomic_delegate_only(file: &syn::File) -> bool {
    let mut has_default = false;
    let mut delegate_impls = false;

    for item in &file.items {
        if let syn::Item::Impl(impl_block) = item {
            if impl_block.trait_.is_some() {
                if ImplBlock(impl_block).is_default_impl() {
                    has_default = true;
                }
                continue;
            }
            if rules::test_usage::TestUsage::is_default_only_impl(impl_block) {
                has_default = true;
            } else if rules::test_usage::TestUsage::is_likely_delegate_impl(impl_block) {
                delegate_impls = true;
            } else {
                return false;
            }
        }
    }

    has_default && delegate_impls
}

#[cfg(test)]
mod tests {
    use super::is_atomic_delegate_only;

    #[test]
    fn test_usage() {
        let with_default: syn::File = syn::parse_quote! {
            impl Default for X { fn default() -> Self { Self } }
            impl X { pub fn greet(&self) -> String { format!("hi") } }
        };
        assert!(is_atomic_delegate_only(&with_default));
        let without_default: syn::File = syn::parse_quote! {
            impl X { pub fn greet(&self) -> String { format!("hi") } }
        };
        assert!(!is_atomic_delegate_only(&without_default));
    }
}
