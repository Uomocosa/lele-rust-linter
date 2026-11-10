pub fn is_default_only_impl(impl_block: &syn::ItemImpl) -> bool {
    for item in &impl_block.items {
        if let syn::ImplItem::Fn(method) = item {
            if method.sig.ident != "default" {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

// no test_usage necessary
