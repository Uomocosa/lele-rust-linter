use crate::rules;
use crate::Visibility;

pub fn collect_pub_items(file: &syn::File) -> Vec<rules::atomic_file::PubItem> {
    let mut items = Vec::new();
    for item in &file.items {
        match item {
            syn::Item::Struct(s) if Visibility(&s.vis).is_exposed() => {
                items.push(rules::atomic_file::PubItem {
                    name: s.ident.to_string(),
                    kind: rules::atomic_file::PubItemKind::Struct,
                });
            }
            syn::Item::Enum(e) if Visibility(&e.vis).is_exposed() => {
                items.push(rules::atomic_file::PubItem {
                    name: e.ident.to_string(),
                    kind: rules::atomic_file::PubItemKind::Enum,
                });
            }
            syn::Item::Fn(f) if Visibility(&f.vis).is_exposed() => {
                items.push(rules::atomic_file::PubItem {
                    name: f.sig.ident.to_string(),
                    kind: rules::atomic_file::PubItemKind::Fn,
                });
            }
            _ => {}
        }
    }
    items
}

// no test_usage necessary
