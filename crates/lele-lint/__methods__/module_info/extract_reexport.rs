use crate::Reexport;

pub fn extract_reexport(tree: &syn::UseTree) -> Option<Reexport> {
    match tree {
        syn::UseTree::Path(p) => {
            let mut segments = vec![p.ident.to_string()];
            if let Some(child) = extract_reexport(&p.tree) {
                segments.extend(child.segments);
                Some(Reexport {
                    segments,
                    is_glob: child.is_glob,
                })
            } else {
                None
            }
        }
        syn::UseTree::Name(n) => Some(Reexport {
            segments: vec![n.ident.to_string()],
            is_glob: false,
        }),
        syn::UseTree::Glob(_) => Some(Reexport {
            segments: Vec::new(),
            is_glob: true,
        }),
        syn::UseTree::Rename(r) => Some(Reexport {
            segments: vec![r.ident.to_string()],
            is_glob: false,
        }),
        syn::UseTree::Group(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::extract_reexport;

    #[test]
    fn test_usage() {
        let named: syn::UseTree = syn::parse_str("player::Player").unwrap();
        let reexport = extract_reexport(&named).unwrap();
        assert_eq!(reexport.segments, vec!["player", "Player"]);
        assert!(!reexport.is_glob);
        let glob: syn::UseTree = syn::parse_str("player::*").unwrap();
        assert!(extract_reexport(&glob).unwrap().is_glob);
    }
}
