use syn::visit::Visit;

use crate::fn_body::control_finder::ControlFinder;
use crate::FnBody;

pub fn has_control_keyword(block: FnBody) -> bool {
    let mut finder = ControlFinder::default();
    finder.visit_block(&block);
    *finder
}

#[cfg(test)]
mod tests {
    use crate::FnBody;

    #[test]
    fn test_usage() {
        let control: syn::Block = syn::parse_quote! {{ while x { y() } }};
        assert!(FnBody(&control).has_control_keyword());
        let plain: syn::Block = syn::parse_quote! {{ iffy(format) }};
        assert!(!FnBody(&plain).has_control_keyword());
        let text: syn::Block = syn::parse_quote! {{ println!("waiting for peers") }};
        assert!(!FnBody(&text).has_control_keyword());
        let closure: syn::Block =
            syn::parse_quote! {{ items.iter().map(|x| if *x > 0 { 1 } else { 0 }).sum() }};
        assert!(FnBody(&closure).has_control_keyword());
    }
}
