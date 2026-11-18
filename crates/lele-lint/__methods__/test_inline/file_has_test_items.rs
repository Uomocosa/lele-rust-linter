use crate::ItemMarker;

pub fn file_has_test_items(file: &syn::File) -> bool {
    let mut pending: Vec<&syn::Item> = file.items.iter().collect();
    while let Some(item) = pending.pop() {
        let attrs: &[syn::Attribute] = match item {
            syn::Item::Const(item) => &item.attrs,
            syn::Item::Enum(item) => &item.attrs,
            syn::Item::ExternCrate(item) => &item.attrs,
            syn::Item::Fn(item) => &item.attrs,
            syn::Item::ForeignMod(item) => &item.attrs,
            syn::Item::Impl(item) => &item.attrs,
            syn::Item::Macro(item) => &item.attrs,
            syn::Item::Mod(item) => &item.attrs,
            syn::Item::Static(item) => &item.attrs,
            syn::Item::Struct(item) => &item.attrs,
            syn::Item::Trait(item) => &item.attrs,
            syn::Item::TraitAlias(item) => &item.attrs,
            syn::Item::Type(item) => &item.attrs,
            syn::Item::Union(item) => &item.attrs,
            syn::Item::Use(item) => &item.attrs,
            syn::Item::Verbatim(_) | _ => &[],
        };
        if attrs
            .iter()
            .any(|attr| ItemMarker(attr).is_cfg_test() || ItemMarker(attr).is_test())
        {
            return true;
        }
        match item {
            syn::Item::Mod(module) => {
                if let Some((_, items)) = &module.content {
                    pending.extend(items.iter());
                }
            }
            syn::Item::Impl(block) => {
                for inner in &block.items {
                    let syn::ImplItem::Fn(method) = inner else {
                        continue;
                    };
                    if method
                        .attrs
                        .iter()
                        .any(|attr| ItemMarker(attr).is_cfg_test() || ItemMarker(attr).is_test())
                    {
                        return true;
                    }
                }
            }
            _ => {}
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::file_has_test_items;
    use crate::rules;

    #[test]
    fn test_usage() {
        let with_mod_tests: syn::File =
            syn::parse_quote! { #[cfg(test)] mod tests { #[test] fn test_usage() {} } };
        assert!(file_has_test_items(&with_mod_tests));
        assert!(rules::test_inline::TestInline::file_has_test_items(
            &with_mod_tests
        ));
        let with_bare_test: syn::File = syn::parse_quote! { #[test] fn case() {} };
        assert!(file_has_test_items(&with_bare_test));
        let nested: syn::File = syn::parse_quote! { mod outer { #[test] fn case() {} } };
        assert!(file_has_test_items(&nested));
        let plain: syn::File = syn::parse_quote! { pub fn extra_tests() -> bool { true } };
        assert!(!file_has_test_items(&plain));
    }
}
