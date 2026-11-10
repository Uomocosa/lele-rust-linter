use crate::rules;

pub fn visit_impl_item_fn<'ast>(
    finder: &mut rules::no_inner_fns::InnerFnFinder<'ast>,
    item: &'ast syn::ImplItemFn,
) {
    finder.fn_depth = finder.fn_depth.saturating_add(1);
    syn::visit::visit_impl_item_fn(finder, item);
    finder.fn_depth = finder.fn_depth.saturating_sub(1);
}

#[cfg(test)]
mod tests {
    use super::visit_impl_item_fn;
    use crate::rules::no_inner_fns::InnerFnFinder;

    #[test]
    fn test_usage() {
        let item: syn::ImplItemFn = syn::parse_str("fn method() { fn helper() {} }").unwrap();
        let mut finder = InnerFnFinder::default();
        visit_impl_item_fn(&mut finder, &item);
        assert_eq!(finder.inner_fns.len(), 1);
    }
}
