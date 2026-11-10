use crate::rules;
use crate::FnBody;

pub fn all_methods_are_real_reserved(impl_block: &syn::ItemImpl) -> bool {
    let mut has_fn = false;
    for item in &impl_block.items {
        let syn::ImplItem::Fn(method) = item else {
            continue;
        };
        has_fn = true;
        if !rules::delegate_macro::DelegateMacro::is_reserved(&method.sig.ident) {
            return false;
        }
        if method.block.stmts.is_empty() || FnBody(&method.block).is_delegate_call() {
            return false;
        }
    }
    has_fn
}

// no test_usage necessary
