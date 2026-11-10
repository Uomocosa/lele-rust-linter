use crate::FnBody;

pub fn is_likely_delegate_impl(impl_block: &syn::ItemImpl) -> bool {
    for item in &impl_block.items {
        if let syn::ImplItem::Fn(method) = item {
            if !FnBody(&method.block).is_short() {
                return false;
            }
        }
    }
    !impl_block.items.is_empty()
}

// no test_usage necessary
