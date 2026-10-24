use crate::rules;

pub fn is_atomic_delegate_only(file: &syn::File) -> bool {
    let mut has_default = false;
    let mut delegate_impls = false;

    for item in &file.items {
        if let syn::Item::Impl(impl_block) = item {
            if impl_block.trait_.is_some() {
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

// no test_usage necessary
