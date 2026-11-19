use std::path::Path;

use rules::test_usage_shape::PrimaryKind;

use crate::rules;
use crate::Origin;

pub fn primary_item(
    file: &syn::File,
    rel_path: &Path,
    origin: Origin,
) -> Option<rules::test_usage_shape::PrimaryItem> {
    if matches!(origin, Origin::Methods) {
        let method = rules::methods_layout::MethodsLayout::file_stem(rel_path)?;
        return Some(rules::test_usage_shape::PrimaryItem {
            kind: PrimaryKind::Method,
            name: method,
            method_names: Vec::new(),
        });
    }

    let mut method_names = Vec::new();
    for item in &file.items {
        let syn::Item::Impl(block) = item else {
            continue;
        };
        for inner in &block.items {
            if let syn::ImplItem::Fn(function) = inner {
                method_names.push(function.sig.ident.to_string());
            }
        }
    }

    let proc_macro_fns: Vec<String> = file
        .items
        .iter()
        .filter_map(|item| {
            let syn::Item::Fn(function) = item else {
                return None;
            };
            let is_entry_point = function.attrs.iter().any(|attr| {
                attr.path().is_ident("proc_macro")
                    || attr.path().is_ident("proc_macro_attribute")
                    || attr.path().is_ident("proc_macro_derive")
            });
            is_entry_point.then(|| function.sig.ident.to_string())
        })
        .collect();
    let pub_items: Vec<rules::atomic_file::PubItem> =
        rules::atomic_file::AtomicFile::collect_pub_items(file)
            .into_iter()
            .filter(|item| {
                !(matches!(item.kind, rules::atomic_file::PubItemKind::Fn)
                    && proc_macro_fns.contains(&item.name))
            })
            .collect();
    let is_plugin_file = rel_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .is_some_and(|stem| stem.ends_with("_plugin"));
    if is_plugin_file {
        if let Some(plugin) = pub_items.iter().find(|item| item.name.ends_with("Plugin")) {
            return Some(rules::test_usage_shape::PrimaryItem {
                kind: PrimaryKind::Plugin,
                name: plugin.name.clone(),
                method_names,
            });
        }
    }

    let primary = pub_items.into_iter().next()?;
    let kind = match primary.kind {
        rules::atomic_file::PubItemKind::Struct => PrimaryKind::Struct,
        rules::atomic_file::PubItemKind::Enum => PrimaryKind::Enum,
        rules::atomic_file::PubItemKind::Fn => PrimaryKind::Fn,
    };
    Some(rules::test_usage_shape::PrimaryItem {
        kind,
        name: primary.name,
        method_names,
    })
}

#[cfg(test)]
mod tests {
    use super::primary_item;
    use crate::rules;
    use crate::Origin;
    use std::path::Path;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_quote! {
            pub struct Guest(pub String);

            impl Guest {
                pub fn greet(&self) -> String { format!("hi") }
            }
        };
        let primary = primary_item(&file, Path::new("guest.rs"), Origin::Src).unwrap();
        assert_eq!(primary.name, "Guest");
        assert!(matches!(
            primary.kind,
            rules::test_usage_shape::PrimaryKind::Struct
        ));
        assert_eq!(primary.method_names, vec!["greet".to_string()]);
        assert!(rules::test_usage_shape::TestUsageShape::primary_item(
            &file,
            Path::new("guest.rs"),
            Origin::Src
        )
        .is_some());
    }

    #[test]
    fn test_usage_takes_the_first_pub_item() {
        let file: syn::File = syn::parse_quote! {
            pub fn compute(value: u32) -> u32 { value * 2 }
            pub struct Later;
        };
        let primary = primary_item(&file, Path::new("compute.rs"), Origin::Src).unwrap();
        assert_eq!(primary.name, "compute");
        assert!(matches!(
            primary.kind,
            rules::test_usage_shape::PrimaryKind::Fn
        ));
    }

    #[test]
    fn test_usage_reads_a_method_file_by_origin() {
        let file: syn::File = syn::parse_quote! {
            pub fn increment(counter: &mut u8) { *counter += 1; }
        };
        let primary = primary_item(
            &file,
            Path::new("src/counter/increment.rs"),
            Origin::Methods,
        )
        .unwrap();
        assert_eq!(primary.name, "increment");
        assert!(matches!(
            primary.kind,
            rules::test_usage_shape::PrimaryKind::Method
        ));
    }

    #[test]
    fn test_usage_finds_the_plugin_struct() {
        let file: syn::File = syn::parse_quote! {
            pub struct P2PPlugin;

            impl P2PPlugin {
                pub fn build(&self) -> u32 { 1 }
            }
        };
        let primary = primary_item(&file, Path::new("p2p_plugin.rs"), Origin::Src).unwrap();
        assert_eq!(primary.name, "P2PPlugin");
        assert!(matches!(
            primary.kind,
            rules::test_usage_shape::PrimaryKind::Plugin
        ));
    }

    #[test]
    fn test_usage_skips_proc_macro_entry_points() {
        let file: syn::File = syn::parse_quote! {
            #[proc_macro]
            pub fn rule_examples(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
                input
            }

            #[proc_macro_attribute]
            pub fn atomic_delegates(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
                item
            }

            #[proc_macro_derive(Guest)]
            pub fn guest_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
                input
            }

            pub(crate) struct ExampleFolderScan;
        };
        let primary = primary_item(&file, Path::new("lib.rs"), Origin::Src).unwrap();
        assert_eq!(primary.name, "ExampleFolderScan");
        assert!(matches!(
            primary.kind,
            rules::test_usage_shape::PrimaryKind::Struct
        ));
    }

    #[test]
    fn test_usage_skips_an_impl_only_file() {
        let file: syn::File = syn::parse_quote! {
            impl Output {
                pub fn render(&self) -> String { String::new() }
            }
        };
        assert!(primary_item(&file, Path::new("output.rs"), Origin::Src).is_none());
    }
}
