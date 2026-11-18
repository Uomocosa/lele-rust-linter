use syn::visit::Visit;

use crate::fn_body::branch_finder::BranchFinder;
use crate::rules;

pub fn is_plain_data(file: &syn::File) -> bool {
    let has_struct_or_enum = file
        .items
        .iter()
        .any(|item| matches!(item, syn::Item::Struct(_) | syn::Item::Enum(_)));
    if !has_struct_or_enum {
        return false;
    }
    if rules::test_usage::TestUsage::is_atomic_delegate_only(file) {
        return false;
    }
    for item in &file.items {
        let syn::Item::Impl(impl_block) = item else {
            continue;
        };
        let is_plugin = impl_block.trait_.as_ref().is_some_and(|(_, path, _)| {
            path.segments
                .last()
                .is_some_and(|seg| seg.ident == "Plugin")
        });
        if is_plugin {
            return false;
        }
        for impl_item in &impl_block.items {
            match impl_item {
                syn::ImplItem::Const(_) | syn::ImplItem::Type(_) => {}
                syn::ImplItem::Fn(method) => {
                    let name = method.sig.ident.to_string();
                    if name != "new" && name != "default" && name != "from" {
                        return false;
                    }
                    let mut finder = BranchFinder::default();
                    finder.visit_block(&method.block);
                    if *finder {
                        return false;
                    }
                }
                _ => return false,
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::is_plain_data;

    #[test]
    fn test_usage() {
        let plain: syn::File = syn::parse_quote! {
            pub struct Settings { pub port: u16 }
            impl Settings { pub fn new(port: u16) -> Self { Self { port } } }
        };
        assert!(is_plain_data(&plain));
        let behaviour: syn::File = syn::parse_quote! {
            pub struct Guest(pub String);
            impl Guest { pub fn greet(&self) -> String { format!("hi") } }
        };
        assert!(!is_plain_data(&behaviour));
    }

    #[test]
    fn test_usage_plugin_impl_is_not_plain() {
        let plugin: syn::File = syn::parse_quote! {
            pub struct RegistryPlugin;
            impl Plugin for RegistryPlugin { fn build(&self, app: &mut App) {} }
        };
        assert!(!is_plain_data(&plugin));
    }

    #[test]
    fn test_usage_match_in_new_is_not_plain() {
        let branching: syn::File = syn::parse_quote! {
            pub struct RetryPolicy { pub attempts: u32 }
            impl RetryPolicy {
                pub fn new(attempts: u32) -> Self {
                    let clamped = match attempts { 0 => 1, n => n };
                    Self { attempts: clamped }
                }
            }
        };
        assert!(!is_plain_data(&branching));
    }

    #[test]
    fn test_usage_no_type_is_not_plain() {
        let free: syn::File = syn::parse_quote! {
            pub fn compute(x: u32) -> u32 { x * 2 }
        };
        assert!(!is_plain_data(&free));
    }
}
