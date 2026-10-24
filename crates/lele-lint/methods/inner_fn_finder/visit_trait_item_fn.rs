use crate::rules;

pub fn visit_trait_item_fn<'ast>(
    finder: &mut rules::no_inner_fns::InnerFnFinder<'ast>,
    item: &'ast syn::TraitItemFn,
) {
    finder.fn_depth = finder.fn_depth.saturating_add(1);
    syn::visit::visit_trait_item_fn(finder, item);
    finder.fn_depth = finder.fn_depth.saturating_sub(1);
}

#[cfg(test)]
mod tests {
    use super::visit_trait_item_fn;
    use crate::rules::no_inner_fns::InnerFnFinder;

    #[test]
    fn test_usage() {
        let item: syn::TraitItemFn = syn::parse_str("fn provided() { fn helper() {} }").unwrap();
        let mut finder = InnerFnFinder::default();
        visit_trait_item_fn(&mut finder, &item);
        assert_eq!(finder.inner_fns.len(), 1);
    }
}
