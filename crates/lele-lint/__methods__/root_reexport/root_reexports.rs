use std::collections::HashSet;

pub fn root_reexports(lib: &syn::File) -> HashSet<(String, String)> {
    let mut pairs = HashSet::new();
    for item in &lib.items {
        if let syn::Item::Use(u) = item {
            if !matches!(u.vis, syn::Visibility::Public(_)) {
                continue;
            }
            if let syn::UseTree::Path(path) = &u.tree {
                if let syn::UseTree::Name(name) = path.tree.as_ref() {
                    pairs.insert((path.ident.to_string(), name.ident.to_string()));
                }
            }
        }
    }
    pairs
}

// no test_usage necessary
