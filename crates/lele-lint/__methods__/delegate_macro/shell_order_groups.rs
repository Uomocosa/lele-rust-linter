use crate::rules;
use crate::ImplItemGroup;
use crate::ImplMethod;

pub fn shell_order_groups(impl_block: &syn::ItemImpl) -> Vec<ImplItemGroup> {
    let mut groups: Vec<ImplItemGroup> =
        rules::delegate_macro::DelegateMacro::fn_methods(impl_block)
            .into_iter()
            .map(|method| ImplMethod(method).order_group())
            .collect();
    groups.sort();
    groups.dedup();
    groups
}

#[cfg(test)]
mod tests {
    use super::shell_order_groups;
    use crate::ImplItemGroup;

    #[test]
    fn test_usage() {
        let mixed: syn::ItemImpl =
            syn::parse_str("impl Foo { pub fn now() -> Self {} pub fn run(&self) {} }\n").unwrap();
        assert_eq!(
            shell_order_groups(&mixed),
            vec![ImplItemGroup::Constructor, ImplItemGroup::PubFn]
        );
        let crate_only: syn::ItemImpl =
            syn::parse_str("impl Foo { pub(crate) fn step(&self) {} }\n").unwrap();
        assert_eq!(
            shell_order_groups(&crate_only),
            vec![ImplItemGroup::CrateFn]
        );
    }
}

// no test_usage necessary
