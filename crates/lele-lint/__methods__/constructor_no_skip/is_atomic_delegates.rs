use crate::FnBody;
use crate::ImplBlock;

pub fn is_atomic_delegates(impl_block: &syn::ItemImpl) -> bool {
    if impl_block.items.is_empty() {
        return false;
    }
    if ImplBlock(impl_block).is_default_impl() {
        return false;
    }
    for item in &impl_block.items {
        if let syn::ImplItem::Fn(method) = item {
            if !FnBody(&method.block).is_delegate_call() {
                return false;
            }
        }
    }
    true
}

// no test_usage necessary
