use crate::rules;
use lele_snake_case::ToLeleSnakeCase;
use std::collections::HashSet;

pub fn missing_fn_flatten(stem: &str, file: &syn::File, lib: &syn::File) -> Vec<String> {
    let mut fns = Vec::new();
    for item in &file.items {
        if let syn::Item::Fn(f) = item {
            if !matches!(f.vis, syn::Visibility::Public(_)) {
                continue;
            }
            if f.sig.ident.to_string().to_snake_case() == stem {
                fns.push(f.sig.ident.to_string());
            }
        }
    }
    if fns.is_empty() {
        return Vec::new();
    }
    let has_private_mod = lib.items.iter().any(|item| {
        matches!(item, syn::Item::Mod(m) if m.ident == stem && matches!(m.vis, syn::Visibility::Inherited))
    });
    let reexported: HashSet<(String, String)> =
        rules::root_reexport::RootReexport::root_reexports(lib);
    let mut missing = Vec::new();
    for name in fns {
        if !has_private_mod {
            missing.push(format!(
                "SHAPE-F fn-file `{stem}.rs` holds `pub fn {name}` — declare private `mod {stem};` in lib.rs (not `pub mod`)"
            ));
        }
        if !reexported.contains(&(stem.to_string(), name.clone())) {
            missing.push(format!(
                "SHAPE-F fn-file `{stem}.rs` holds `pub fn {name}` — add `pub use {stem}::{name};` to lib.rs for `crate::{name}()`"
            ));
        }
    }
    missing
}

// no test_usage necessary
