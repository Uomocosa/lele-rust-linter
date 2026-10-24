use crate::rules;
use crate::ItemMarker;

pub fn visit_item_mod<'ast>(
    finder: &mut rules::no_inner_fns::InnerFnFinder<'ast>,
    item: &'ast syn::ItemMod,
) {
    if item.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test()) {
        return;
    }
    syn::visit::visit_item_mod(finder, item);
}

#[cfg(test)]
mod tests {
    use super::visit_item_mod;
    use crate::rules::no_inner_fns::InnerFnFinder;

    #[test]
    fn test_usage() {
        let tests: syn::ItemMod =
            syn::parse_str("#[cfg(test)] mod tests { fn t() { fn helper() {} } }").unwrap();
        let inline: syn::ItemMod =
            syn::parse_str("mod inline { fn f() { fn helper() {} } }").unwrap();
        let mut finder = InnerFnFinder::default();
        visit_item_mod(&mut finder, &tests);
        assert_eq!(finder.inner_fns.len(), 0);
        visit_item_mod(&mut finder, &inline);
        assert_eq!(finder.inner_fns.len(), 1);
    }
}
