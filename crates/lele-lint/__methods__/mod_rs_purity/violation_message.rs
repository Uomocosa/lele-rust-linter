use crate::rules;
use crate::ItemMarker;

pub fn violation_message(item: &syn::Item) -> Option<String> {
    match item {
        syn::Item::Mod(module) if module.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test()) => Some(
            "mod.rs may contain only `mod`/`pub mod` declarations and `pub use` re-exports — `#[cfg(test)] mod` test modules are not allowed"
                .to_string(),
        ),
        syn::Item::Mod(_) => None,
        syn::Item::Use(item_use) => {
            if matches!(item_use.vis, syn::Visibility::Inherited) {
                Some(
                    "mod.rs may contain only `pub use` re-exports — private `use` imports are not allowed; re-export with `pub use` or move the import into a function file"
                        .to_string(),
                )
            } else {
                None
            }
        }
        syn::Item::Struct(s) => Some(rules::mod_rs_purity::ModRsPurity::named_message("struct", &s.ident.to_string(), &s.vis)),
        syn::Item::Enum(e) => Some(rules::mod_rs_purity::ModRsPurity::named_message("enum", &e.ident.to_string(), &e.vis)),
        syn::Item::Fn(f) => Some(rules::mod_rs_purity::ModRsPurity::named_message("fn", &f.sig.ident.to_string(), &f.vis)),
        syn::Item::Const(c) => Some(rules::mod_rs_purity::ModRsPurity::named_message("const", &c.ident.to_string(), &c.vis)),
        syn::Item::Static(s) => Some(rules::mod_rs_purity::ModRsPurity::named_message("static", &s.ident.to_string(), &s.vis)),
        syn::Item::Trait(t) => Some(rules::mod_rs_purity::ModRsPurity::named_message("trait", &t.ident.to_string(), &t.vis)),
        syn::Item::Type(t) => Some(rules::mod_rs_purity::ModRsPurity::named_message("type alias", &t.ident.to_string(), &t.vis)),
        syn::Item::Impl(_) => Some(
            "mod.rs may contain only `mod`/`pub mod` declarations and `pub use` re-exports — `impl` blocks are not allowed"
                .to_string(),
        ),
        syn::Item::Macro(_) => Some(
            "mod.rs may contain only `mod`/`pub mod` declarations and `pub use` re-exports — macro definitions are not allowed"
                .to_string(),
        ),
        _ => Some(
            "mod.rs may contain only `mod`/`pub mod` declarations and `pub use` re-exports — move this item to its own file"
                .to_string(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let imp: syn::Item = syn::parse_quote! { impl Thing {} };
        assert!(rules::mod_rs_purity::ModRsPurity::violation_message(&imp)
            .is_some_and(|message| message.contains("`impl` blocks")));
        let module: syn::Item = syn::parse_quote! { pub mod alpha; };
        assert!(rules::mod_rs_purity::ModRsPurity::violation_message(&module).is_none());
    }
}
